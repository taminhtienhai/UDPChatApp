# Communication App with UDP connection written in Rust

## What is UDP?

UDP(User Datagram Protocal) is the way two device connect and transfer data between. Compare to TCP, this lack of trusted because data can be lost in comming way, but much faster.

UDP prefer to create video call application or game online, because these don't really need data integrity. That why you can see online stream video sometime missed frames or lag while playing online game.

## Why this app?

Basic understand about how network really working. Just a 'Hello world' app that perform data transfer, enjoy.

## Run Server

```shell
cd ./server
cargo run
```

## Run Client

```shell
cd ./client
cargo run
```

> After run, switch to server CLI to see message just sent.

