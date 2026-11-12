# scour

Agregador de **recon** para pentest autorizado: DNS, subdomínios, portas e
whois num comando só, com **saída JSON limpa** pra pipar no `jq`/Elastic.

> ⚠️ **Uso autorizado apenas.** Rode só contra alvos para os quais você tem
> permissão explícita. Uso não autorizado é crime; a responsabilidade é sua.

## Módulos
- `--dns` — A/AAAA/MX/NS/TXT/SOA (ou PTR reverso se o alvo for IP).
- `--subs` — brute de subdomínios (wordlist embutida ou `--wordlist`).
- `--ports` — connect scan das portas comuns + banner (`--port-list` pra custom).
- `--whois` — consulta porta 43 seguindo o `refer:` do IANA.
- `-A/--all` — tudo. Sem flags: `dns` + `whois` (rápido e não intrusivo).

## Uso
```sh
scour exemplo.com -A -o recon.json
scour exemplo.com --subs --wordlist subs.txt -c 400
scour 10.0.0.5 --ports --port-list 22,80,443,3389
```

## Build
```sh
cargo build --release
```

Stack: tokio, hickory-dns, clap, serde.
