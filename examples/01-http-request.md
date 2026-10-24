# A simple HTTP request

The smallest useful diagram: a browser, an API and a database.

Shows: circle and box nodes, solid and dashed arrows, labels, a built-in icon.

```plotum
# plotum 1
title: A simple HTTP request

browser (Browser)
api [API | Node.js]
db [PostgreSQL] icon=database

browser -> api : GET /users/42
api -> db : SELECT * FROM users
db --> api : 1 row
api --> browser : 200 OK
```
