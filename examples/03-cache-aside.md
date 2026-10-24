# Cache-aside

Before and after adding a Redis cache in front of the database.

Shows: `bad` and `ok` sections side by side, named colors.

```plotum
# plotum 1
title: Cache-aside
subtitle: Read from Redis first, fall back to the database

section bad "No cache"
  api [API]
  db [PostgreSQL] icon=database
  api -> db {red} : every read, about 40 ms

section ok "Cache-aside"
  api2 [API]
  redis [Redis | TTL 5 min]
  db2 [PostgreSQL] icon=database
  api2 -> redis : 1. GET user-42
  api2 -> db2 : 2. on miss, SELECT
  api2 --> redis : 3. SET user-42
  note: a hit takes about 1 ms, a miss pays the database once
```
