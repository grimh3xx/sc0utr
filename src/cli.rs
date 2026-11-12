use clap::Parser;

/// scour — agregador de recon (DNS / subdomínios / portas / whois).
///
/// Uso autorizado apenas. Rode só contra alvos com permissão explícita.
#[derive(Parser, Debug, Clone)]
#[command(name = "scour", version, about, long_about = None)]
pub struct Cli {
    /// Alvo: domínio (exemplo.com) ou IP.
    pub target: String,

    /// Roda todos os módulos.
    #[arg(short = 'A', long)]
    pub all: bool,

    /// Resolução DNS (A/AAAA/MX/NS/TXT/SOA).
    #[arg(long)]
    pub dns: bool,

    /// Brute de subdomínios (usa wordlist embutida ou --wordlist).
    #[arg(long)]
    pub subs: bool,

    /// Arquivo de wordlist para subdomínios (um por linha).
    #[arg(long)]
    pub wordlist: Option<String>,

    /// Scan de portas comuns (connect scan + banner).
    #[arg(long)]
    pub ports: bool,

    /// Lista de portas custom, ex: 22,80,443,8080 (padrão: top comuns).
    #[arg(long)]
    pub port_list: Option<String>,

    /// Consulta whois (porta 43, segue referral do IANA).
    #[arg(long)]
    pub whois: bool,

    /// Concorrência máxima.
    #[arg(short, long, default_value_t = 200)]
    pub concurrency: usize,

    /// Timeout por operação de rede (ms).
    #[arg(long, default_value_t = 3000)]
    pub timeout: u64,

    /// Grava resultado JSON no arquivo.
    #[arg(short, long)]
    pub output: Option<String>,

    /// Silencia o progresso no stderr.
    #[arg(short, long)]
    pub quiet: bool,
}

impl Cli {
    /// Se nenhum módulo foi escolhido, roda um perfil padrão sensato.
    pub fn resolved_modules(&self) -> (bool, bool, bool, bool) {
        if self.all {
            return (true, true, true, true);
        }
        let any = self.dns || self.subs || self.ports || self.whois;
        if !any {
            // padrão: dns + whois (rápido, não intrusivo)
            (true, false, false, true)
        } else {
            (self.dns, self.subs, self.ports, self.whois)
        }
    }
}
