#### Start from scratch here

You may follow these steps:

1. Create a Rust project, preferably a workspace, and cd into it.

`mkdir axum_workspace` and `cd axum_workspace`

2. Create a workspace by first creating a `Cargo.toml` file inside the just created directory `axum_workspace` with the following similar content:

```toml
[workspace]
members = [
    "axum_postgres_docker",
]
exclude = [
    "docs"
]
```

3. Create the first member crate inside `axum_workspace` named as `axum_postgres_docker`:

```
cargo new --bin axum_postgres_docker
```

Also change directory into `axum_postgres_docker` using command `cd axum_postgres_docker`.

4. Create a migrations directory inside `axum_postgres_docker` and add its initial boilerplate file (SQL code for your code logic) if available. Feel free to use the following setup SQL code as your first migration:

```sql
-- This is a boilerplate migration file that we use in nearly every project.
-- It sets up database features that we use quite often.

-- As a style choice, we prefer not to write SQL in all uppercase as lowercase feels friendlier to the eyes.
-- It's nicer to read WHEN THE CODE ISN'T YELLING AT YOU ALL DAY.
-- It perhaps made sense back when code highlighting was not the norm and case was used to differentiate keywords
-- from non-keywords, but at this point it's purely from inertia.
-- The language itself is not case-sensitive except for quoted identifiers.
-- Whichever style you use, however, consistency should still be maintained.

-- This extension gives us `uuid_generate_v1mc()` which generates UUIDs that cluster better than `gen_random_uuid()`
-- while still being difficult to predict and enumerate.
-- Also, while unlikely, `gen_random_uuid()` can in theory produce collisions which can trigger spurious errors on
-- insertion, whereas it's much less likely with `uuid_generate_v1mc()`.
create extension if not exists "uuid-ossp";

-- We try to ensure every table has `created_at` and `updated_at` columns, which can help immensely with debugging
-- and auditing.
--
-- While `created_at` can just be `default now()`, setting `updated_at` on update requires a trigger which
-- is a lot of boilerplate. These two functions save us from writing that every time as instead we can just do
--
-- select trigger_updated_at('<table name>');
--
-- after a `CREATE TABLE`.
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;

-- Finally, this is a text collation that sorts text case-insensitively, useful for `UNIQUE` indexes
-- over things like usernames and emails, without needing to remember to do case-conversion.
create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
```

Please note that the above `SQL` setup code was grabbed from the `Realworld-axum-sqlx` project found here - https://github.com/launchbadge/realworld-axum-sqlx/

<pre>IMPORTANT: Make sure you have the sqlx-cli installed on your local development machine</pre>

To install sqlx-cli with postgres feature:

```sh
cargo install sqlx-cli --features postgres
```

5. Generate a build script. Make sure you generate the build script inside member `axum_postgres_docker` root-dir level. The command to use is:

```shell
sqlx migrate build-script
```

N.B:
Make sure to push the resulting `build.rs` file to source-control.

5. Create a `.env` file inside member crate `axum_postgres_docker`'s root directory. You may use the following credentials as a starter template:

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

6. Add crate dependencies before creating source-code:

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
cargo add tracing-subsriber
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

7. Apply (write/copy) the code inside member crate `axum_postgres_docker` of this workspace repo. Here is the general gist of member crate `axum_postgres_docker`'s code:

`Code structure`:

<pre>
axum_postgres_docker
  |- migrations
  |   |- 0001-setup.sql
  |- src
  |   |- http
  |   |   |- root_path.rs
  |   |- config.rs
  |   |- main.rs
  |   |- run_database.rs
  |- .dockerignore
  |- .env
  |- .env.sample
  |- build.rs
  |- Cargo.toml
  |- docker-compose.yml
  |- Dockerfile
</pre>

And the project's code execution flow looks like this:

<pre>
         ___ <code>config.rs</code>
        |
<code>main.rs</code>
        |___ <code>run_database.rs</code>
        |
        |___ <code>http/mod.rs</code>
</pre>

- <pre>src/config.rs</pre>

`config.rs` - Contains a struct called `Config` with two String fields, `database_url` and `hmac_key`. `Config`'s implementation has a function `new` whose default implementation returns a `Config` wherein values for its two fields, `database_url` and `hmac_key` are supplied from `.env` variables.

```rs
// src/config.rs
#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub hmac_key: String,
}

impl Config {
    pub fn new() -> Config {
        // -------- DATABASE SETUP START
        // load up '.env'
        dotenv::dotenv().ok(); // 'dotenv' is a third-party crate

        // extract `DATABASE_URL` value inside now loaded '.env' file
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found.");

        // extract `HMAC_KEY` value inside now loaded '.env' file
        let hmac_key = std::env::var("HMAC_KEY").expect("HMAC_KEY not found!");

        Config {
            database_url,
            hmac_key,
        }
    }
}
```

- <pre>src/run_database.rs</pre>

`run_database.rs` - Contains function `run_database()` that accepts a non-optional String value. This string value is expected to be a valid Postgres database url, and then returns an `Ok(db_pool_options)` variant of `Result<Pool<Postgres>`, which is a connection to a live. In this project the Postgres database is provided as a Docker container with a docker volume for data persistence.

Inside function `run_database()`, `db_pool_options` allows up to 50 max async connections, and invokes macro `sqlx::migrate!()` against the live Postgres database connection `db_pool_options` for runtime migration. You could manually execute migrations using sqlx-cli tool command `sqlx db setup`.

```rs
// src/run_database.rs
use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn run_database(database_url: &String) -> Result<Pool<Postgres>> {
    // create a PgPoolOptions connection to DATABASE_URL
    let db_pool_options = PgPoolOptions::new()
        .max_connections(50)
        .connect(database_url)
        .await
        .with_context(|| "Connection to Database failed")?;

    // Correctly embed into this application database migrations
    // ...performed by command `$ sqlx db setup`
    // ...You can use this command each time you add to dir migrations
    sqlx::migrate!()
        .run(&db_pool_options)
        .await
        .with_context(|| "SQLx DB migration failed")?;

    Ok(db_pool_options)
}
```

- <pre>src/http/mod.rs</pre>

`mod.rs` - Contains function `build_routes()` that takes two non-option arguments; a `Config` and a `PgPool` live database connection instance. It takes ownership of these two arguments to create an `ApiContext` instance, which it then passes to another function `api_router()`.

Function `api_router()` uses its single `ApiContext` instance argument to create a `Router` that contains all http-routes nested or merged in with it, and then provides the `ApiContext` instance as a state to go along with it. `api_router()` returns its `Router` instance when function `build_routes()` calls it.

```rs
// src/http/mod.rs
mod add_new_expense;
mod root_path;

use crate::config::Config;

use add_new_expense::add_new_expense;
use axum::{
    routing::{get, post},
    Router,
};
use root_path::get_root_path;
use sqlx::PgPool; // Newly added!
                  //use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[allow(unused)]
#[derive(Clone)]
pub(crate) struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub fn build_routes(config: Config, db: PgPool) -> Router {
    let api_context = ApiContext {
        config: Arc::new(config),
        db,
    };

    let app = api_router(api_context);

    app
}

fn api_router(api_context: ApiContext) -> Router {
    Router::new()
        .route("/", get(get_root_path))
        .route("/create-new-expense", post(add_new_expense))
        .with_state(api_context)
}
```

- <pre>src/main.rs</pre>

`main.rs` - Contains function `start_app()` which initiates logging for debugging using crate `tracing_subscriber`. Function `start_app()` calls `Config::new()` which creates a new `Config` instance, it calls function `run_database()` from `run_database.rs` to create a new live `PgPool` connection and run migrations on the database, it also specifies a socket address which it should bind the axum server to, as in, `axum::Server::bind(&socket_addr)`, and then append the returned `Router` from called function `build_routes()` inside `mod.rs` as a service to function `serve()`, as in; `axum::Server::bind(&socket_addr).serve(routes_aggregate.into_make_service())`.

```rs
// src/main.rs
mod config;
mod http; // DIR
mod run_database; // Sibling-Module

use anyhow::{Context, Result};
use config::Config;
use http::build_routes;
use run_database::run_database;
use std::error::Error; // not strictly required here since anyhow is utilized.
use std::net::SocketAddr;
use tracing;
use tracing_subscriber;

pub async fn start_app() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let config = Config::new();

    let db = run_database(&config.database_url).await.unwrap();

    // -------- SERVER INIT START
    // Fn http::build_routes()' inside 'http/mod.rs' is my own little alternative to 'http::serve(config, db)' from Realworld API
    let routes_aggregate = build_routes(config, db);

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], 8090));

    // run axum server on localhost:8090
    axum::Server::bind(&socket_addr)
        .serve(routes_aggregate.into_make_service())
        .await
        .with_context(|| "Server failed to start/serve")?;

    tracing::debug!(
        "This axum server started successfully and is now listening on {}",
        socket_addr
    );
    // -------- SERVER INIT END

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    start_app().await.unwrap();
    Ok(())
}
```

- <pre>src/http/root_path.rs</pre>

`root_path.rs` - Contains function `root_path()` which returns an `Html<String>` which implements axum's `IntoResponse`. Function `root_path()` takes no argument, and outputs HTML with the message `Welcome! This Rust axum app is now running at Port 8090`.

```rs
// src/http/root_path.rs
use axum::response::Html;

pub async fn get_root_path() -> Html<String> {
    Html("
    <div style='width: auto; height: auto; background: #eee; padding: 2rem; text-align: center; border: 5px solid white;'>
        <div style='flex: 1 100%;'>
            <p>Welcome! This Rust axum app is now running at Port 8090</p>
        </div>
    <div>".to_owned(),
    )
}
```

<h6>The remaining non-Rust files that this project requires you to create at the bearest minimum are: </h6>

- <pre>Dockerfile</pre>

`Dockerfile` - Contains instructions for the Docker engine to use and create a new custom docker image for the Rust `axum` API.

```Dockerfile
FROM rust:1.71.0-slim as builder

WORKDIR /usr/src

# Create blank project. Remember to change the project's name
RUN USER=root cargo new axum_postgres_docker

# We want dependencies cached, so copy those (Cargo.toml, Cargo.lock) first
## PLEASE NOTE: I did not copy 'Cargo.lock' because this particular project is a workspace member without its own 'Cargo.lock'
COPY Cargo.toml /usr/src/axum_postgres_docker/

# Set the working directory
WORKDIR /usr/src/axum_postgres_docker

# Install 'musl-tools' to enable successful image build
RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get -y install musl-tools

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/axum_postgres_docker/src/

# Copy your migrations and file 'build.rs' too
COPY migrations /usr/src/axum_postgres_docker/migrations
COPY build.rs /usr/src/axum_postgres_docker

## Touch mainlrs to prevent cached release build
RUN touch /usr/src/axum_postgres_docker/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:3.16.0 AS runtime

# Copy application binary from builder image
COPY --from=builder /usr/src/axum_postgres_docker/target/x86_64-unknown-linux-musl/release/axum_postgres_docker /usr/local/bin

EXPOSE 8090

# Run the application via 'axum_postgres_docker.exe'
CMD ["/usr/local/bin/axum_postgres_docker"]
```

- <pre>axum_postgres_docker/docker-compose.yml</pre>

`docker-compose.yml` - Contains instructions for the Docker engine to create a Postgres container, and another container based off a Docker image built atop the Rust `axum` API.

```yaml
version: '3.9'

services:
  app-db:
    image: postgres:latest
    restart: always
    env_file: .env
    environment:
      - POSTGRES_PASSWORD=${POSTGRES_PWD}
      - POSTGRES_USER=${POSTGRES_USR}
      - POSTGRES_DB=${POSTGRES_DB}
    ports:
      - ${DB_PORT}
    volumes:
      - db-data:/var/lib/postgresql/data
    healthcheck:
      test: psql -U postgres -q -d postgres -c "SELECT 'ready';"
      interval: 10s
      timeout: 5s
      retries: 5
      start_period: 5s
  axum_backend:
    #build: .
    image: axum_postgres_docker:1.0
    env_file: .env
    ports:
      - ${AXUM_SERVER_PORT}
    depends_on:
      app-db:
        condition: service_healthy

volumes:
  db-data: {}
```

- <pre>axum_postgres_docker/.env</pre>

dd a `.env` file which would contain credentials for your app, such as your database url and server port address which should be used inside your `docker-compose.yml`

```.env
POSTGRES_USR=postgres
POSTGRES_PWD=a-password
POSTGRES_DB=app-db
DB_PORT=5432:5432

# This DATABASE_URL does not work inside docker, but should always work on your dev-machine, provided that pgadmin does not get in the way of host 'localhost'
#DATABASE_URL=postgres://postgres:a-password@localhost/postgres

DATABASE_URL=postgres://${POSTGRES_USR}:${POSTGRES_PWD}@app-db:5432/${POSTGRES_DB}

AXUM_SERVER_PORT=8090:8090

HMAC_KEY=a-random-key
```

Alternatively, you can utilize the `.env.sample` file that accompany this project inside directory `axum_postgres_docker`, using command:

```sh
cp ./.env.sample ./.env
```

- Now is the time to init your project as a Git repo at the root level of the workspace, so that Git can capture `axum_postgres_docker` and your frontend member crate e.g, `leptos_frontend_docker`.

Inside `axum_workspace`:

```sh
git init
```

Since you have now initialized the entire workspace as a Git repo, you can make sure to have a `.gitignore` file, and perhaps a `README.md` file. This template guide would not provide any `README.md` content, it is up to you to decide the decription of your own custom project moving forward. Below is a template `.gitignore` file for this project.

- <pre>axum_postgres_docker/.gitignore</pre>

`.gitignore` - Contains files and directories that should be ignored when you push your local Git repo (on your dev-machine) to a remote Git repo like https://GitHub.com

```.gitignore
/target
/axum_postgres_docker/.env
```

8. Generate a build script. Make sure you generate the build script inside member `axum_postgres_docker` root-dir level. The command to use is:

```shell
sqlx migrate build-script
```

N.B:
Make sure to push the resulting `build.rs` file to source-control.

9. Set up your database migrations using the `sqlx-cli` tool. If you add new migrations to (i.e, add new `.sql` files), re-setup your database migrations again using the `sqlx cli tool`. The command to set up migrations is -

```sh
sqlx db setup
```

OR

```sh
sqlx database setup
```

10. Run `cargo build`. Refer back to the repo's README.md for instructions on how to build a docker image for the project, and spin-up docker containers.
