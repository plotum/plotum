# REST API for one resource

One resource, five endpoints, with the status code each one returns.

Shows: sections, box subtext, a note.

```plotum
# plotum 1
title: REST API for orders
subtitle: One resource, five verbs

section info "Client"
  app (Mobile app)

section ok "Orders API"
  list [GET | /orders]
  create [POST | /orders]
  read [GET | /orders/42]
  update [PATCH | /orders/42]
  remove [DELETE | /orders/42]
  app -> list : 200 OK
  app -> create : 201 Created
  app -> read : 200 OK
  app -> update : 200 OK
  app -> remove : 204 No Content
  note: GET is safe, DELETE is idempotent, POST is neither
```
