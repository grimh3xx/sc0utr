use anyhow::Result;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

const IANA: &str = "whois.iana.org";

async fn ask(server: &str, query: &str, to: Duration) -> Result<String> {
    let mut stream = timeout(to, TcpStream::connect((server, 43))).await??;
    stream
        .write_all(format!("{query}\r\n").as_bytes())
        .await?;
    stream.flush().await?;
    let mut out = String::new();
    timeout(to, stream.read_to_string(&mut out)).await??;
    Ok(out)
}

/// Consulta IANA e segue o `refer:` para o servidor autoritativo.
pub async fn lookup(target: &str, to: Duration) -> Result<String> {
    let root = ask(IANA, target, to).await?;
    let refer = root
        .lines()
        .find_map(|l| l.trim().strip_prefix("refer:").map(|s| s.trim().to_string()));
    match refer {
        Some(server) if !server.is_empty() => {
            let detail = ask(&server, target, to).await.unwrap_or_default();
            if detail.trim().is_empty() {
                Ok(root)
            } else {
                Ok(format!("% referral: {server}\n{detail}"))
            }
        }
        _ => Ok(root),
    }
}
