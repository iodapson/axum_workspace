- Create a Rust project, preferably a workspace

- Create a workspace by first creating a Cargo.toml file with the following similar content:

```toml
[workspace]
members = [
    "axum_postgres_docker",
    "letpos_frontend"
]
exclude = [
    "steps"
]
```

- Now run `cargo build` from inside your workspace's root directory.

- Create a migrations directory inside `axum_postgres_docker` and add its initial boilerplate file (SQL code for your code logic) if available

- Creat a '.env' file inside member crate `axum_workspace`'s root directory.

- Create your database credentials inside `.env`. You can use the following as a starter template:

```.env
POSTGRES_USR=postgres
POSTGRES_PWD=a-password
POSTGRES_DB=appdb

# This DATABASE_URL does not work inside docker, but should always work on your dev-machine, provided that pgadmin does not get in the way of host 'localhost'
#DATABASE_URL=postgres://postgres:a-password@localhost/postgres

DATABASE_URL=postgres://${POSTGRES_USR}:${POSTGRES_PWD}@host.docker.internal:5432/${POSTGRES_DB}
```

- Dependency installations before adding source-code:

`cargo add axum`
`cargo add tokio -F "full"`
`cargo add sqlx -F "postgres" -F "runtime-tokio-rustls"`
`cargo add serde -F "derive"`
`cargo add tracing`
`cargo add tracing-subsriber`
`cargo add dotenv`
`cargo add anyhow`

- N.B: At this point you want to make sure you already have sqlx-cli:

`cargo install sqlx-cli`

- Apply the code inside member crate `axum_postgres_docker` of inside this workspace repo.
