# Multi-server chat with Kafka

Why WebSocket servers need a broker to see each other's messages.

Shows: theme gradients and custom icons, steps.

```plotum
# plotum 1
title: Multi-server chat
subtitle: Why use a broker
footer: plotum.dev

theme:
  colors:
    brand: #6D28D9
    accent: #EC4899
  gradients:
    hero: brand -> accent
  icons:
    kafka: https://example.com/kafka.svg

section bad "Without a broker"
  alice (Alice)
  s1 [Server 1 | WebSocket]
  s2 [Server 2 | WebSocket]
  alice -> s1 : message
  s1 ..> s2 {red} : never sees the message

section ok "With Kafka"
  wrap: 4
  bob (Bob) step=1
  k [Kafka | topic chat] icon=kafka {hero} step=2
  bob -> k step=2 : publish
  k -> s2 {brand} step=3 : consume
  note: every server reads the same topic
```
