use anyhow::Result;
use std::collections::HashSet;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

const IANA: &str = "whois.iana.org";
const MAX_HOPS: usize = 5;

async fn ask(server: &str, query: &str, to: Duration) -> Result<String> {
    let mut stream = timeout(to, TcpStream::connect((server, 43))).await??;
    stream.write_all(format!("{query}\r\n").as_bytes()).await?;
    stream.flush().await?;
    let mut out = String::new();
    timeout(to, stream.read_to_string(&mut out)).await??;
    Ok(out)
}

/// Extracts the next whois server from a response (`refer:`, `whois:`, or
/// ARIN-style `ReferralServer: whois://host`).
fn next_server(resp: &str) -> Option<String> {
    for line in resp.lines() {
        let t = line.trim();
        for key in ["refer:", "whois:", "ReferralServer:"] {
            if let Some(v) = t.strip_prefix(key) {
                let v = v.trim().trim_start_matches("whois://").trim_start_matches("rwhois://");
                let host = v.split([':', '/']).next().unwrap_or("").trim();
                if !host.is_empty() {
                    return Some(host.to_string());
                }
            }
        }
    }
    None
}

/// Queries IANA and follows the referral chain to the authoritative server,
/// up to MAX_HOPS, returning the most specific (last) response.
pub async fn lookup(target: &str, to: Duration) -> Result<String> {
    let mut server = IANA.to_string();
    let mut seen = HashSet::new();
    let mut last = String::new();
    let mut chain = Vec::new();
    for _ in 0..MAX_HOPS {
        if !seen.insert(server.clone()) {
            break;
        }
        let resp = match ask(&server, target, to).await {
            Ok(r) => r,
            Err(_) if !last.is_empty() => break,
            Err(e) => return Err(e),
        };
        chain.push(server.clone());
        last = resp.clone();
        match next_server(&resp) {
            Some(s) if s != server => server = s,
            _ => break,
        }
    }
    Ok(format!("% whois chain: {}\n{last}", chain.join(" -> ")))
}

#[cfg(test)]
mod tests {
    use super::next_server;
    #[test]
    fn parses_referral_forms() {
        assert_eq!(next_server("refer: whois.verisign-grs.com\n").as_deref(), Some("whois.verisign-grs.com"));
        assert_eq!(next_server("ReferralServer: whois://whois.arin.net\n").as_deref(), Some("whois.arin.net"));
        assert_eq!(next_server("no referral here\n"), None);
    }
}
