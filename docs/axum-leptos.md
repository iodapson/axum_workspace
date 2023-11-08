** Still Under Construction **

## Start from scratch here

You may follow these steps:

1. Make sure `cargo-leptos` is installed on your local dev machine. If unsure, still run the command to install it - the installation command would either refuse to install or update to a new version if your already have `cargo-leptos` installed, but run the command to install it. The command to install it is:

```sh
cargo install cargo-leptos
```

2. Create a new directory (`<your-project's-root-directory>`) to contain all your Leptos-Axum code. And clone a template workspace inside from the <code>Leptos</code> repo on GitHub with command:

```sh
cargo leptos new --git https://github.com/Gentle/start-axum-workspace
```

OR

```sh
cargo leptos new --git https://github.com/leptos-rs/start-axum-workspace
```

You'll be asked to provide a name for the project. In this repo's case, it was named `start-axum-workspace`. Feel free to provide a different name, it doesn't matter since it will only exist temporarily.

3. This step may not be applicable to you.

Since cloning the `start-axum-workspace` project (https://github.com/Gentle/start-axum-workspace) essentially nests a Git repo inside the directory (`<your-project's-root-directory>`) chosen to be the root-dir for your code project with its own separate Git initialization, you will have to delete the nested git repo inside directory `start-axum-workspace`, and then copy all of its content into `<your-project's-root-directory>`.

To delete the Git repo inside `start-axum-workspace`, enter the following command;

```sh
cd start-axum-workspace && rm -rf .git
```

4. Now move-out all of the contents inside sub-workspace `start-axum-workspace` directly into `<your-project's-root-directory` then delete the now empty `start-axum-workspace` directory. The consecutive commands to use are:

```sh
mv * ..
```

---

<h6>N.B:</h6>

- In case you `$ cargo build` or `$ cargo run` the code inside `start-axum-workspace` before moving it out into `<your-project's-root-directory>`, make sure that you delete the `target` sub-directory inside `start-axum-workspace`. Delete `target` with command;

```sh
rm -rf target
```

- In case `<your-project's-root-directory>` is already a Git repo with its own `.gitnore` file before you attempted to move the contents of `start-axum-workspace` into it, manually move the `.gitgnore` file inside `start-axum-workspace` into `<your-project's-root-directory` since it'll get ignored by the `mv` command used earlier inside `start-axum-workspace`. Replace any prior `.gitignore` file.

---

<h6>Don't forget to remove the now empty 'start-axum-workspace' directory, using command:</h6>

```sh
rm -rf start-axum-workspace
```

5. Inside `<your-project's-root-directory>` root level `Cargo.toml`, change parameter:

```toml
[[workspace.metadata.leptos]]
# this name is used for the wasm, js and css file names
name = "start-axum-workspace"
```

(from line 35 or so)

..to:

```toml
[[workspace.metadata.leptos]]
# this name is used for the wasm, js and css file names
name = "<your-project's-root-directory>"
```

6. Initialize a new Git repo inside `<your-project's-root-directory>` using Git commands;

```sh
git init
```

```sh
git add *
```

```sh
git commit -m "Initial files for '<your-project's-root-directory>'"
```

And then rename the branch from `master` to something more fitting, such as `main` or `base`. This guide assumes that you have a Dev-machine-local Git branch named `main`.

```sh
git branch -m main
```

7. Create a new exclude list inside `<your-project's-root-directory`'s `Cargo.toml`. The exclude list comprise directories - `axum_postgres_docker`(actually ignore this one), and `docs`. Sub-directory `docs` is useful for keeping notes like this very one that you're examining.

You can add the following to line 3 of `<your-project's-root-directory>`'s `Cargo.toml` file:

```toml

exclude = [
    "docs",
    "axum_postgres_docker", # Ignore this particular line
]

```

8. Feel free to create an empty sub-directory `docs` right inside `<your-project's-root-directory>`. At this stage, this guide assumes that `<your-project's-root-directory` contains the following files & sub-directories:

<pre>
<code>
app
docs
end2end
frontend
public
server
style
Cargo.toml
LICENSE
README.md
rust-toolchain.toml
</code>
</pre>

9. Edit the `rust-toolchain.toml` file inside `your-project's-root-directory`'s root-dir level to specify that the rust `nightly` toolchain be used to compile the entire workspace. Some leptos code extras require it. You should also include a `wasm32-unknown-unknown` target. Your `rust-toolchain.toml` ought to look like this;

```rust-toolchain.toml
[toolchain]
channel = "nightly"
targets = ["wasm32-unknown-unknown"]
```

10. Right inside `<your-project's-root-diretory>`, create a `.github/workflows/rust.yml`, such that events 'on.push' and 'on.pull_request' now target Git branch `start-axum-workspace`. You may use the following as the contents of `rust.yml`:

```yml
name: Rust

on:
  push:
    branches: ['main']
  pull_request:
    branches: ['main']

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3
      - name: Build
        run: cargo build --verbose
      - name: Run tests
        run: cargo test --verbose
```

11. Optionally make changes inside the `Cargo.toml` of corresponsing `app`, `frontend`, and `server` member-crates of `axum_workspace`. You'll find suggestions as to what these changes are.

For `<your-project's-root-directory>/app/Cargo.toml`, edit its content to become;

```toml
[package]
name = "app"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
leptos = {workspace = true}
leptos_meta = {workspace = true}
leptos_router = {workspace = true}
leptos_axum = { workspace = true, optional = true }

http = {workspace = true}
cfg-if = {workspace = true}
thiserror = {workspace = true}

[features]
default = []
hydrate = ["leptos/hydrate", "leptos_meta/hydrate", "leptos_router/hydrate"]
ssr = ["leptos/ssr", "leptos_meta/ssr", "leptos_router/ssr", "dep:leptos_axum"]

```

For `<your-project's-root-directory>/frontend/Cargo.toml`, edit its content to become;

```toml
[package]
name = "frontend"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
app = { path = "../app", default-features = false, features = ["hydrate"] }
leptos = { workspace = true, features = [ "hydrate" ] }

console_error_panic_hook = {workspace = true}
console_log = {workspace = true}
log = {workspace = true}
wasm-bindgen = {workspace = true}

```

For `your-project's-root-directory>/server/Cargo.toml`, edit its content to become;

```toml
[package]
name = "server"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
app = { path = "../app", default-features = false, features = ["ssr"] }
leptos = { workspace = true, features = [ "ssr" ]}
leptos_axum = {workspace = true}

axum = {workspace = true}
simple_logger = {workspace = true}
tokio = {workspace = true}
tower = {workspace = true}
tower-http = {workspace = true}
log = {workspace = true}

```

12. Run `cargo build` anywhere inside in the `<your-project's-root-directory>` directory.

Note: At this stage, you might encounter a compilation error that looks similar to this -

```
...(head, tail) = html_parts_separated(options, use_context::<MetaC...
| ^^^^^^^^^^^^^^^^^^^^ ------- an argument of type `leptos::Scope` is missing

...)
```

If you follow the link to the module causing the build error, you'll see the following faulty `html_parts_seperated()` function call in line `647` -

```rs
let (head, tail) = html_parts_separated(options, use_context::<MetaContext>(cx).as_ref());
```

_Fixing the error_

<h6>A Temporary Fix</h6>:

The temporary simple fix to this error as suggested by the Rust compiler itself is to add a new argument of type `leptos::Scope` as the first argument to `html_parts_separated()` function call, at line `647`.

The fix should look as follows:

```rs
let (head, tail) = html_parts_separated(cx, options, use_context::<MetaContext>(cx).as_ref());
```

_N.B_:

<h6>The Permanent Fix:</h6>

A more permanent fix is to edit the entire workspace, as in `<your-project's-root-directory>`'s `Cargo.toml` file and change the `leptos_axum` version dependency from its default `version = 0.4` value to a more explicit version, as in `version = 0.4.10`.
Fix the error.

_The last time I used the `start-axum-workspace` template, I did not have apply this fix because it came with the template. But take note of issues like this which may be fixed by verifying your crates dependency version(s)._

13. Fix `cargo build` warning: 'some crates are on edtion 2021 which defaults to `resolver = "2"`, but virtual workspaces default to `resolver = "1"`. To fix this warning, edit `<your-project's-root-directory>`'s `Cargo.toml` file and add property;

```toml
resolver = "2"
```

_You can enter a new line at line 3 and add the above line to the Cargo.toml file specified_

14. Run command to launch leptos app at the `axum_workspace` root-level dir:

```sh
cargo leptos watch
```

It may take a while to compile. You may want to take a 15 mins break. Believe it or not, you have pretty far.

15. Once cargo leptos is down compiling your entire workspace, open your favorite browser and check-out url `localhost:3000`.

---

<h5>Dockerize Your Entire Application</h5>

16. Return to your VS Code editor, and add new Dockerfile from this link - https://github.com/leptos-rs/leptos-website/blob/main/Dockerfile
    Copy and paste the Dockerfile you find inside this link. Please edit the Dockerfile to adapt it to the official `start-axum-workspace` template like so:

```Dockerfile
# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bullseye as builder

# If you’re using stable, use this instead
# FROM rust:1.70-bullseye as builder

# Install 'musl-tools' to enable successful image build ** New addition here
RUN apt-get -y update \
  && apt-get -y upgrade \
  && apt-get -y install musl-tools
# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
# ** changes made here, copy the right files.
COPY ./app ./frontend ./server ./style ./Cargo.lock ./Cargo.toml ./rust_toolchain.toml .

# Build the app
RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-bullseye as runner
# Copy the server binary to the /app directory ** changes included here
COPY --from=builder /app/target/server/release/server /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
# ** choose any prot you want, I chose 8070
ENV LEPTOS_SITE_ADDR="0.0.0.0:8070"
ENV LEPTOS_SITE_ROOT="site"
# ** Expose the same port for your 'LEPTOS_SITE_ADDR' env variable
EXPOSE 8070
# Run the server ** changes included here
CMD ["/app/server"]
```

17. Create new `docker-compose.yml` file to spin up two containers, one for a Postgres database, and the other for the axum-leptos Docker image

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
  axum_leptos:
    #build: .
    image: axum_leptos_docker:1.0
    env_file: .env
    ports:
      - ${AXUM_SERVER_PORT}
    depends_on:
      app-db:
        condition: service_healthy

volumes:
  db-data: {}
```

18. Create a `.env` file to supply credentials needed by the Postgres container. You can make use of this `.env.sample`

```.env
POSTGRES_USR=postgres
POSTGRES_PWD=a-password
POSTGRES_DB=app-db
DB_PORT=5432:5432

# This DATABASE_URL does not work inside docker, but should always work on your dev-machine, provided that pgadmin does not get in the way of host 'localhost'
#DATABASE_URL=postgres://postgres:a-password@localhost/postgres

DATABASE_URL=postgres://${POSTGRES_USR}:${POSTGRES_PWD}@app-db:5432/${POSTGRES_DB}

AXUM_SERVER_PORT=8070:8070

HMAC_KEY=a-random-key
```

19. Now refer back to the README.md of this branch to find instructions to run this app.
