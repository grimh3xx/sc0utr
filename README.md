# sc0utr

[![build](https://img.shields.io/github/actions/workflow/status/grimh3xx/sc0utr/build.yml?branch=main&style=flat-square&labelColor=111826&label=build)](https://github.com/grimh3xx/sc0utr/actions)
[![license](https://img.shields.io/github/license/grimh3xx/sc0utr?style=flat-square&labelColor=111826&color=22D3EE)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-2021-3B82F6?style=flat-square&labelColor=111826)
![platform](https://img.shields.io/badge/Linux-x64-22D3EE?style=flat-square&labelColor=111826)
![authorised use](https://img.shields.io/badge/authorised%20use-only-F87171?style=flat-square&labelColor=111826)


**Agregador de recon em Rust para avaliações autorizadas.** Junta num só comando o
primeiro passo de todo engajamento — DNS (A/AAAA/MX/NS/TXT/SOA e PTR reverso),
brute de subdomínios, connect-scan de portas com banner e whois seguindo o
`refer:` do IANA — com **saída JSON limpa** pra pipar no `jq`/Elastic.

<p align="center"><img src="docs/demo.svg" alt="sc0utr demo" width="100%"></p>

> ## ⚠️ Uso autorizado apenas
> Recon ativo (DNS brute, portscan) toca o alvo. Rode **somente** contra sistemas
> que você tem **permissão explícita** para testar. Uso não autorizado é crime; a
> responsabilidade é inteiramente sua.

## Módulos

| flag | o que faz |
|---|---|
| `--dns` | A/AAAA/MX/NS/TXT/SOA (ou PTR reverso se o alvo for IP) |
| `--subs` | brute de subdomínios (wordlist embutida ou `--wordlist`) |
| `--ports` | connect scan das portas comuns + banner (`--port-list` pra custom) |
| `--whois` | consulta porta 43 seguindo o `refer:` do IANA |
| `-A/--all` | tudo. Sem flags: `dns` + `whois` (rápido e não intrusivo) |

## Uso

```sh
sc0utr exemplo.com -A -o recon.json
sc0utr exemplo.com --subs --wordlist subs.txt -c 400
sc0utr 10.0.0.5 --ports --port-list 22,80,443,3389
```

Alvo aceita domínio ou IP. Concorrência com `-c`, timeout com `--timeout`.

## Build

```sh
cargo build --release
```

Regenerar o demo: `python3 tools/gen_demo.py`.

## Licença

MIT. Veja [LICENSE](LICENSE).
