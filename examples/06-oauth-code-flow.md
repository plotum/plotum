# OAuth 2.0 authorization code flow

How a web app gets an access token without ever seeing the user's password.

Shows: a longer flow split into steps, dashed responses.

```plotum
# plotum 1
title: OAuth 2.0 authorization code flow

user (User)
app [Web app | OAuth client]
auth [Auth server | login + consent]
api [Resource API]

user -> app step=1 : click Log in
app -> auth step=2 : redirect with client_id
user -> auth step=3 : log in and consent
auth --> app step=4 : redirect with code
app -> auth step=5 : exchange code + secret
auth --> app step=5 : access token
app -> api step=6 : Bearer token
```
