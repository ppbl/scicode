# scicode

Code and science blog, written in rust.

client side use [yew](https://yew.rs/) + [trunk](https://trunkrs.dev/)

server side use [actix-web](https://actix.rs/) + [diesel](https://diesel.rs/) + [postgres](https://www.postgresql.org/)

## client

`cargo install trunk`  
`trunk serve --open`

## server

`cargo install cargo-watch`  
`cargo watch -x 'run --bin server'`
