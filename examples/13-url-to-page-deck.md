# What happens when you type a URL

A presentation deck: DNS, TLS, CDN and origin, one step at a time.

Shows: a full deck with header, footer, dark mode, a custom font, light/dark
colors, a gradient and six steps.

```plotum
# plotum 1
title: What happens when you type a URL
subtitle: From the address bar to the first byte
footer: plotum.dev

theme:
  mode: dark
  font: jetbrains-mono
  colors:
    brand: #EA580C / #FB923C
    accent: #CA8A04 / #FACC15
  gradients:
    hero: brand -> accent

section info "Network"
  browser (Browser) step=1
  dns [DNS resolver | 1.1.1.1] step=2
  cdn [CDN edge | TLS 1.3] step=3
  origin [Origin server | nginx] step=5

  browser -> dns step=2 : where is example.com?
  dns --> browser step=2 : 93.184.215.14
  browser <-> cdn step=3 : TLS handshake
  browser -> cdn step=4 : GET /
  cdn -> origin step=5 : cache miss
  origin --> browser {hero} step=6 : 200 OK + HTML
  note: on a cache hit, the CDN answers at step 4
```
