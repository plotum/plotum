# Idempotent payments

Why a retry after a timeout can charge a customer twice, and how an idempotency
key prevents it.

Shows: `bad` and `ok` sections telling a before/after story, hex colors.

```plotum
# plotum 1
title: Idempotent payments
subtitle: Safe retries with an idempotency key

section bad "Retry without a key"
  app [Shop]
  psp [Payment API]
  app -> psp : POST /charges
  app -> psp {#EF4444} : retry after timeout
  note: the customer is charged twice

section ok "Retry with a key"
  app2 [Shop]
  psp2 [Payment API]
  keys [Key store | 24 h]
  app2 -> psp2 : POST /charges, key abc
  psp2 -> keys : save key and result
  app2 -> psp2 {#22C55E} : retry, same key
  psp2 --> app2 : stored result, no new charge
```
