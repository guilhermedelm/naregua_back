# Naregua Back — guia do repositório

Este arquivo é o mapa compacto do projeto para assistentes de código. Consulte os
arquivos-fonte apenas quando a tarefa exigir detalhes de implementação.

## Visão geral

- Backend HTTP escrito em Rust (edition 2021).
- Framework web: Axum 0.8, executado pelo Tokio.
- Serialização: Serde e `serde_json`.
- CORS: `tower-http`, atualmente permissivo.
- Observabilidade disponível: `tracing` e `tracing-subscriber`.
- Manifesto e dependências: `Cargo.toml`.

## Estrutura e responsabilidades

```text
src/
├── main.rs                  # inicialização do servidor e composição da aplicação
├── state.rs                 # estado compartilhado (ainda vazio)
├── db/
│   ├── mod.rs               # expõe o módulo users
│   └── users.rs             # acesso a dados de usuários (ainda vazio)
├── handlers/
│   ├── mod.rs               # expõe o módulo users
│   └── users.rs             # handlers HTTP de usuários (ainda vazio)
├── routes/
│   ├── mod.rs               # expõe o módulo users
│   └── users.rs             # declara as rotas de usuários
└── services/
    └── user_service.rs      # regras de negócio de usuários (ainda vazio)
```

Arquitetura pretendida:

```text
requisição HTTP -> routes -> handlers -> services -> db
                                      \-> state compartilhado
```

As camadas devem permanecer separadas: rotas fazem o roteamento, handlers tratam
HTTP, services concentram regras de negócio e `db` trata persistência.

## Endpoints declarados

O `main.rs` monta as rotas de usuários sob `/users`:

| Método | Caminho completo     | Handler         |
|--------|----------------------|-----------------|
| GET    | `/users/`            | `hello`         |
| POST   | `/users/register`    | `register_user` |

## Estado atual importante

O código ainda não compila como está. Antes de assumir que a base está funcional,
considere estes pontos:

- `main.rs` declara `mod models`, mas não há `src/models.rs` nem `src/models/mod.rs`.
- `main.rs` ainda não declara os módulos `db`, `services` e `state`.
- `routes/users.rs` referencia `hello` e `register_user`, mas eles não estão
  definidos nem importados.
- Há um atributo `#[tokio::main]` indevido sobre `rotas`.
- O endereço de bind está escrito como `0.0.0.0.3000`; o formato esperado é
  `0.0.0.0:3000`.
- Alguns módulos estão vazios e representam somente a estrutura planejada.

## Convenções para alterações

- Siga o fluxo `routes -> handlers -> services -> db` ao adicionar funcionalidades.
- Coloque tipos de domínio e DTOs no módulo `models` quando ele for criado.
- Passe dependências compartilhadas por um estado Axum definido em `state.rs`.
- Não coloque regra de negócio em arquivos de rotas.
- Prefira erros tipados e conversão explícita para respostas HTTP; evite `unwrap`
  fora da inicialização quando houver falha recuperável.
- Atualize este mapa quando módulos, endpoints, dependências ou decisões de
  arquitetura mudarem.

## Comandos de verificação

```bash
cargo fmt --check
cargo check
cargo test
```

