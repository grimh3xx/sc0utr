mod cli;
mod dns;
mod output;
mod ports;
mod subs;
mod whois;

use clap::Parser;
use cli::Cli;
use output::Report;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

macro_rules! info {
    ($quiet:expr, $($arg:tt)*) => {
        if !$quiet { eprintln!("[*] {}", format!($($arg)*)); }
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let (do_dns, do_subs, do_ports, do_whois) = cli.resolved_modules();
    let to = Duration::from_millis(cli.timeout);
    let target_ip: Option<IpAddr> = cli.target.parse().ok();

    let mut report = Report {
        target: cli.target.clone(),
        ..Default::default()
    };

    let resolver = Arc::new(dns::resolver());

    if do_dns {
        info!(cli.quiet, "DNS: resolvendo {}", cli.target);
        let res = match target_ip {
            Some(ip) => dns::reverse(&resolver, ip).await,
            None => dns::forward(&resolver, &cli.target).await,
        };
        report.dns = Some(res);
    }

    if do_whois {
        info!(cli.quiet, "WHOIS: consultando {}", cli.target);
        match whois::lookup(&cli.target, to).await {
            Ok(w) => report.whois = Some(w),
            Err(e) => info!(cli.quiet, "whois falhou: {e}"),
        }
    }

    if do_subs {
        if target_ip.is_some() {
            info!(cli.quiet, "subs: alvo é IP, pulando brute de subdomínio");
        } else {
            let words: Vec<String> = match &cli.wordlist {
                Some(path) => std::fs::read_to_string(path)?
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty() && !l.starts_with('#'))
                    .collect(),
                None => subs::BUILTIN.iter().map(|s| s.to_string()).collect(),
            };
            info!(cli.quiet, "subs: testando {} candidatos", words.len());
            report.subdomains =
                subs::brute(resolver.clone(), &cli.target, &words, cli.concurrency).await;
            info!(cli.quiet, "subs: {} encontrados", report.subdomains.len());
        }
    }

    if do_ports {
        // escolhe um IP alvo: o próprio se for IP, senão o primeiro A do DNS
        let scan_ip: Option<IpAddr> = target_ip.or_else(|| {
            report
                .dns
                .as_ref()
                .and_then(|d| d.a.first())
                .and_then(|a| a.parse().ok())
        });
        match scan_ip {
            Some(ip) => {
                let plist: Vec<u16> = match &cli.port_list {
                    Some(s) => s
                        .split(',')
                        .filter_map(|p| p.trim().parse().ok())
                        .collect(),
                    None => ports::TOP_PORTS.to_vec(),
                };
                info!(cli.quiet, "portas: scaneando {} em {} portas", ip, plist.len());
                report.ports = ports::scan(ip, &plist, cli.concurrency, to).await;
                info!(cli.quiet, "portas: {} abertas", report.ports.len());
            }
            None => info!(cli.quiet, "portas: sem IP resolvível, pulando"),
        }
    }

    let json = serde_json::to_string_pretty(&report)?;
    match &cli.output {
        Some(path) => {
            std::fs::write(path, &json)?;
            info!(cli.quiet, "salvo em {path}");
        }
        None => println!("{json}"),
    }
    Ok(())
}
