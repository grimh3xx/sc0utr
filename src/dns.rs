use crate::output::DnsResult;
use hickory_proto::rr::RecordType;
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;
use std::net::IpAddr;

pub fn resolver() -> TokioAsyncResolver {
    TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
}

async fn query(r: &TokioAsyncResolver, name: &str, rt: RecordType) -> Vec<String> {
    match r.lookup(name, rt).await {
        Ok(lk) => lk
            .record_iter()
            .filter_map(|rec| rec.data().map(|d| d.to_string()))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Coleta registros DNS de um domínio.
pub async fn forward(r: &TokioAsyncResolver, domain: &str) -> DnsResult {
    let (a, aaaa, mx, ns, txt, soa) = tokio::join!(
        query(r, domain, RecordType::A),
        query(r, domain, RecordType::AAAA),
        query(r, domain, RecordType::MX),
        query(r, domain, RecordType::NS),
        query(r, domain, RecordType::TXT),
        query(r, domain, RecordType::SOA),
    );
    DnsResult {
        a,
        aaaa,
        mx,
        ns,
        txt,
        soa: soa.into_iter().next(),
    }
}

/// PTR reverso quando o alvo é um IP.
pub async fn reverse(r: &TokioAsyncResolver, ip: IpAddr) -> DnsResult {
    let mut out = DnsResult::default();
    if let Ok(lk) = r.reverse_lookup(ip).await {
        out.ns = lk.iter().map(|n| n.to_string()).collect();
    }
    out
}
