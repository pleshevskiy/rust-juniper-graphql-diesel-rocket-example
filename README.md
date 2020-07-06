# Rust Juniper GraphQL Diesel Rocket Example

Clone this project and follow the instructions below and start playing today! 
Unlike the [Rocket example](https://github.com/graphql-rust/juniper/blob/master/juniper_rocket/examples/rocket_server.rs) in the Juniper repo, it can be built without relying on types defined in Juniper tests.

## Running the project

Since Rocket uses the latest and greatest features of Rust, you need to use a nightly version of rust. 
As per the [Rocket documentation](https://rocket.rs/guide/getting-started/), you can do this using

```bash
rustup default nightly
```

or per project basis

```bash
rustup override set nightly
```

Need to install diesel_cli for database managing

```bash
cargo install diesel_cli
```

Run example database and run migrations using

```bash
docker-compose -f docker-compose.example.yml up
diesel migration run
```

Lastly, you can run the server using

```bash
cargo run
```

If all goes well, you will be able to visit http://localhost:8000/ with a graphiql interface.



## Attributions


Thank you for examples:

* [Rust-Juniper-Rocket-GraphQL-Example](https://github.com/martimatix/Rust-Juniper-Rocket-GraphQL-Example)
* [rocket-diesel-rest-api-example](https://github.com/sean3z/rocket-diesel-rest-api-example)
* [juniper-example-todo-backend](https://github.com/mhallin/juniper-example-todo-backend)
