# Order saga

A distributed transaction without a global lock: each service undoes its own
work when a later step fails.

Shows: `warn` section, compensations as dotted red links, steps.

```plotum
# plotum 1
title: Order saga
subtitle: Compensating a failed payment

section ok "Forward steps"
  order [Order service] step=1
  stock [Inventory service] step=2
  pay [Payment service] step=3
  ship [Shipping service]
  order -> stock step=2 : reserve items
  stock -> pay step=3 : charge card
  pay -> ship : create shipment

section warn "Payment declined"
  pay ..> stock {red} step=4 : release items
  stock ..> order {red} step=5 : cancel order
  note: shipping is never called, nothing to undo there
```
