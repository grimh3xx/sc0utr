use clap::Parser;

/// sc0utr — recon aggregator (DNS / subdomains / ports / whois).
///
/// Authorised use only. Run only against targets you have explicit permission
/// to test.
#[derive(Parser, Debug, Clone)]
#[command(name = "sc0utr", version, about, long_about = None)]
pub struct Cli {
    /// Target: a domain (example.com) or an IP.
    pub target: String,

    /// Run every module.
    #[arg(short = 'A', long)]
    pub all: bool,

    /// DNS resolution (A/AAAA/MX/NS/TXT/SOA).
    #[arg(long)]
    pub dns: bool,

    /// Subdomain brute force (built-in wordlist or --wordlist).
    #[arg(long)]
    pub subs: bool,

    /// Wordlist file for subdomains (one per line).
    #[arg(long)]
    pub wordlist: Option<String>,

    /// Port scan of common ports (connect scan + banner grab).
    #[arg(long)]
    pub ports: bool,

    /// Custom port list, e.g. 22,80,443,8080 (default: common top ports).
    #[arg(long)]
    pub port_list: Option<String>,

    /// Whois lookup (port 43, follows the IANA referral).
    #[arg(long)]
    pub whois: bool,

    /// Max concurrency.
    #[arg(short, long, default_value_t = 200)]
    pub concurrency: usize,

    /// Per-operation network timeout (ms).
    #[arg(long, default_value_t = 3000)]
    pub timeout: u64,

    /// Max port-scan connections per second (0 = unlimited).
    #[arg(long, default_value_t = 0)]
    pub rate: u32,

    /// Write the JSON report to a file.
    #[arg(short, long)]
    pub output: Option<String>,

    /// Silence progress on stderr.
    #[arg(short, long)]
    pub quiet: bool,
}

impl Cli {
    /// If no module was selected, fall back to a sensible default profile.
    pub fn resolved_modules(&self) -> (bool, bool, bool, bool) {
        if self.all {
            return (true, true, true, true);
        }
        let any = self.dns || self.subs || self.ports || self.whois;
        if !any {
            // default: dns + whois (fast, non-intrusive)
            (true, false, false, true)
        } else {
            (self.dns, self.subs, self.ports, self.whois)
        }
    }
}
