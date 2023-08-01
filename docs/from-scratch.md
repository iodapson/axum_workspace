#### Start from scratch here

You may follow these steps:

1. Create a Rust project, preferably a workspace, and cd into it.

`mkdir axum_workspace` and `cd axum_workspace`

2. Create a workspace by first creating a Cargo.toml file with the following similar content:

```toml
[workspace]
members = [
    "axum_postgres_docker",
    "leptos_frontend"
]
exclude = [
    "docs"
]
```

3. Create the first member crate named as `axum_postgres_docker`.

```
cargo new --bin axum_postgres_docker
```

4. Create a migrations directory inside `axum_postgres_docker` and add its initial boilerplate file (SQL code for your code logic) if available.

5. Create a '.env' file inside member crate `axum_workspace`'s root directory.

6. Create your database credentials inside `.env`. You can use the following as a starter template:

```.env
POSTGRES_USR=postgres
POSTGRES_PWD=a-password
POSTGRES_DB=app-db

# This DATABASE_URL does not work inside docker, but should always work on your dev-machine, provided that pgadmin does not get in the way of host 'localhost'
#DATABASE_URL=postgres://postgres:a-password@localhost/postgres

DATABASE_URL=postgres://${POSTGRES_USR}:${POSTGRES_PWD}@app-db:5432/${POSTGRES_DB}
```

7. Dependency installations before adding source-code:

`cargo add axum`

`cargo add tokio -F "full"`

`cargo add sqlx -F "postgres" -F "runtime-tokio-rustls"`

`cargo add serde -F "derive"`

`cargo add tracing`

`cargo add tracing-subsriber`

`cargo add dotenv`

`cargo add anyhow`

<pre>N.B: At this point you want to make sure you already have sqlx-cli</pre>

`cargo install sqlx-cli`

8. Apply the code inside member crate `axum_postgres_docker` of inside this workspace repo.

9. Run `cargo build`. Refer back to the repo's README.md for instructions on how to build a docker image for the project, and spin-up docker containers.
