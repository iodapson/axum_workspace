#### Start from scratch here

You may follow these steps:

1. Create a Rust project, preferably a workspace, and cd into it.

`mkdir axum_workspace` and `cd axum_workspace`

2. Create a workspace by first creating a `Cargo.toml` file inside the just created `axum_workspace` directory, and provide the following workspace specification content:

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

3. Create the first member crate inside `axum_workspace` named as `axum_postgres_docker`:

```
cargo new --bin axum_postgres_docker
```

Also change your current directory in your terminal to `axum_postgres_docker` using command `cd axum_postgres_docker`

4. Create a migrations directory inside `axum_postgres_docker`, and add its initial boilerplate file (SQL code for your code logic) if available.

5. Create a '.env' file inside member crate `axum_postgres_docker`'s root directory.

6. Create your database credentials inside `.env`. You can use the following as a starter template:

```.env
POSTGRES_USR=postgres
POSTGRES_PWD=a-password
POSTGRES_DB=app-db
DB_PORT=5432:5432

# This DATABASE_URL does not work inside docker, but should always work on your dev-machine, provided that pgadmin does not get in the way of host 'localhost'
#DATABASE_URL=postgres://postgres:a-password@localhost/postgres

DATABASE_URL=postgres://${POSTGRES_USR}:${POSTGRES_PWD}@${DB_PORT}:5432/${POSTGRES_DB}

AXUM_SERVER_PORT=8090:8090
```

7. Add crate dependencies before adding creating source-code:

```
cargo add axum
```

```
cargo add tokio -F "full"
```

```
cargo add sqlx -F "postgres" -F "runtime-tokio-rustls"
```

```
cargo add serde -F "derive"
```

```
cargo add tracing
```

```
cargo add tracing-subscriber
```

```
cargo add dotenv
```

```
cargo add anyhow
```

<pre>N.B: At this point you want to make sure you already have sqlx-cli</pre>

```
cargo install sqlx-cli
```

8. Apply (write/copy) the code inside member crate `axum_postgres_docker` of this workspace repo.

9. Set up your database migrations using the `sqlx-cli` tool. If you add new migrations to (i.e, add new `.sql` files), re-setup your database migrations again using the `sqlx cli tool`. The command to set up migrations is -

```
sqlx db setup
```

OR

```
sqlx database setup
```

9. Run `cargo build`. Refer back to the repo's README.md for instructions on how to build a docker image for the project, and spin-up docker containers.
