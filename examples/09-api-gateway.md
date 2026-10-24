# API gateway in front of microservices

One entry point for every client, routing to the right service.

Shows: multi-line labels, a node created by a link (`events`), double arrows.

```plotum
# plotum 1
title: API gateway

theme:
  colors:
    brand: #0EA5E9

section info "Clients"
  web (Web)
  mobile (Mobile)

section ok "Platform"
  gw [API gateway\nauth + rate limit] {brand}
  users [Users | Go]
  orders [Orders | Rust]
  search [Search | Elasticsearch]
  web -> gw : HTTPS
  mobile -> gw : HTTPS
  gw -> users : /users
  gw -> orders : /orders
  gw -> search : /search
  /* events is never declared:
     the links below create it */
  orders ..> events : OrderCreated
  search <-> events : reindex
```
