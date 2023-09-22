# actix-graphql-template
### crates
- actix-web
- async-graphql
- diesel
- postgres

### how to start
- clone this repo
- run `cp .env.sample .env`
- run `docker compose up`

### sample query
```
query {
    echo(message: "hello")
}
```

### if you do not use diesel or postgres
- edit `db.rs`
