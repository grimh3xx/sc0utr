use crate::output::PortResult;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;

pub const TOP_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 389, 443, 445, 465, 587,
    636, 993, 995, 1433, 1521, 2049, 3306, 3389, 5432, 5900, 5985, 5986, 6379,
    8000, 8080, 8443, 8888, 9000, 9200, 11211, 27017,
];

async fn probe(ip: IpAddr, port: u16, to: Duration) -> PortResult {
    let addr = (ip, port);
    match timeout(to, TcpStream::connect(addr)).await {
        Ok(Ok(mut stream)) => {
            // tenta ler banner curto sem enviar nada (serviços que falam primeiro)
            let mut buf = [0u8; 256];
            let banner = match timeout(Duration::from_millis(1200), stream.read(&mut buf)).await {
                Ok(Ok(n)) if n > 0 => {
                    let s: String = String::from_utf8_lossy(&buf[..n])
                        .chars()
                        .map(|c| if c.is_control() && c != '\n' { ' ' } else { c })
                        .collect();
                    Some(s.trim().to_string())
                }
                _ => None,
            };
            PortResult { port, open: true, banner }
        }
        _ => PortResult { port, open: false, banner: None },
    }
}

pub async fn scan(ip: IpAddr, ports: &[u16], concurrency: usize, to: Duration) -> Vec<PortResult> {
    let sem = Arc::new(Semaphore::new(concurrency));
    let mut handles = Vec::new();
    for &p in ports {
        let sem = sem.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.ok();
            probe(ip, p, to).await
        }));
    }
    let mut open = Vec::new();
    for h in handles {
        if let Ok(r) = h.await {
            if r.open {
                open.push(r);
            }
        }
    }
    open.sort_by_key(|r| r.port);
    open
}
