# actix-graphql-template
### crates
- actix-web
- async-graphql
- diesel
- postgres

### how to start
- clone this repo
- run `docker compose up`

### sample query
```
query {
    echo(message: "hello")
}
```

### if you do not need diesel
- remove `db.rs`
- cleanup `main.rs`
- remove dependencies: `diesel`, `anyhow`, `dotenv`
