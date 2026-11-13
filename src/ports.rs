use crate::output::PortResult;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;

pub const TOP_PORTS: &[u16] = &[
    21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 389, 443, 445, 465, 587,
    636, 993, 995, 1433, 1521, 2049, 3306, 3389, 5432, 5900, 5985, 5986, 6379,
    8000, 8080, 8443, 8888, 9000, 9200, 11211, 27017,
];

/// Global connection rate limiter (max connects/sec), shared across workers.
struct Rate {
    interval: Option<Duration>,
    next: Mutex<Instant>,
}

impl Rate {
    fn new(per_sec: u32) -> Self {
        let interval = if per_sec == 0 {
            None
        } else {
            Some(Duration::from_secs_f64(1.0 / per_sec as f64))
        };
        Rate { interval, next: Mutex::new(Instant::now()) }
    }
    async fn tick(&self) {
        if let Some(iv) = self.interval {
            let now = Instant::now();
            let slot = {
                let mut n = self.next.lock().unwrap();
                let slot = (*n).max(now);
                *n = slot + iv;
                slot
            };
            let d = slot.saturating_duration_since(now);
            if !d.is_zero() {
                tokio::time::sleep(d).await;
            }
        }
    }
}

async fn probe(ip: IpAddr, port: u16, to: Duration) -> PortResult {
    match timeout(to, TcpStream::connect((ip, port))).await {
        Ok(Ok(mut stream)) => {
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

pub async fn scan(
    ip: IpAddr,
    ports: &[u16],
    concurrency: usize,
    to: Duration,
    rate_per_sec: u32,
) -> Vec<PortResult> {
    let sem = Arc::new(Semaphore::new(concurrency));
    let rate = Arc::new(Rate::new(rate_per_sec));
    let mut handles = Vec::new();
    for &p in ports {
        let sem = sem.clone();
        let rate = rate.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.ok();
            rate.tick().await;
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
