# Behind a load balancer

A request from the internet to a pool of web servers in a private network.

Shows: theme colors, links between sections, a failed health check.

```plotum
# plotum 1
title: Behind a load balancer

theme:
  colors:
    brand: #2563EB

section info "Internet"
  user (User)
  dns [DNS | api.example.com]

section ok "Private network 10.0.1.0/24"
  lb [Load balancer | round robin] {brand}
  web1 [Web 1 | 10.0.1.11]
  web2 [Web 2 | 10.0.1.12]
  web3 [Web 3 | 10.0.1.13]
  user -> dns : resolve
  user -> lb {brand} : HTTPS 443
  lb -> web1 : HTTP 8080
  lb -> web2 : HTTP 8080
  lb ..> web3 {red} : health check failed
  note: web3 is out of the pool until it answers again
```
