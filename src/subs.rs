use crate::output::Subdomain;
use hickory_resolver::TokioAsyncResolver;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Minimal built-in wordlist; use --wordlist for anything serious.
pub const BUILTIN: &[&str] = &[
    "www", "mail", "ftp", "dev", "staging", "stage", "test", "api", "admin",
    "portal", "vpn", "remote", "git", "gitlab", "jenkins", "ci", "db", "sql",
    "mysql", "ns1", "ns2", "smtp", "pop", "imap", "webmail", "autodiscover",
    "intranet", "internal", "corp", "ad", "dc", "exchange", "owa", "sharepoint",
    "jira", "confluence", "wiki", "cdn", "static", "assets", "img", "proxy",
    "gateway", "fw", "firewall", "router", "backup", "nas", "store", "shop",
    "beta", "demo", "app", "apps", "cloud", "s3", "files", "docs", "support",
];

pub async fn brute(
    r: Arc<TokioAsyncResolver>,
    domain: &str,
    words: &[String],
    concurrency: usize,
) -> Vec<Subdomain> {
    let sem = Arc::new(Semaphore::new(concurrency));
    let mut handles = Vec::new();
    for w in words {
        let host = format!("{w}.{domain}");
        let r = r.clone();
        let sem = sem.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.ok()?;
            match r.lookup_ip(host.as_str()).await {
                Ok(ips) => {
                    let addrs: Vec<String> = ips.iter().map(|ip| ip.to_string()).collect();
                    if addrs.is_empty() {
                        None
                    } else {
                        Some(Subdomain { name: host, addrs })
                    }
                }
                Err(_) => None,
            }
        }));
    }
    let mut found = Vec::new();
    for h in handles {
        if let Ok(Some(s)) = h.await {
            found.push(s);
        }
    }
    found.sort_by(|a, b| a.name.cmp(&b.name));
    found
}
