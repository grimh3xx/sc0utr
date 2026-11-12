use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct Report {
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<DnsResult>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subdomains: Vec<Subdomain>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<PortResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whois: Option<String>,
}

#[derive(Serialize, Default, Debug)]
pub struct DnsResult {
    pub a: Vec<String>,
    pub aaaa: Vec<String>,
    pub mx: Vec<String>,
    pub ns: Vec<String>,
    pub txt: Vec<String>,
    pub soa: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Subdomain {
    pub name: String,
    pub addrs: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct PortResult {
    pub port: u16,
    pub open: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,
}
