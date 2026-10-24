# TCP three-way handshake

The three packets that open a TCP connection, one per step.

Shows: forced grid positions, steps.

```plotum
# plotum 1
title: TCP three-way handshake

client (Client) @0,0
server (Server) @2,0

client -> server step=1 : SYN seq=100
server -> client step=2 : SYN-ACK seq=300 ack=101
client -> server step=3 : ACK ack=301
note: the connection is open, data can flow both ways
```
