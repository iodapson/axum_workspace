<h4>Welcome!</h4>

<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Starter Template

This is an extension template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

---

## Introduction And Description

This is the start of a new web-app that aims to demo integrating Rust Axum backend with a Rust Leptos frontend, containerized and served on the Cloud on an Alkamai Cloud instance.

Check `axum_workspace/docs/axum-leptos.md` for a detailed code-walkthrough of how to recreate this the code in this branch from scratch.

This repo builds atop an official template - `start-axum-workspace` provided by the Leptos team. You can take a close look at the official `start-axum-workspace` template here: https://github.com/leptos-rs/start-axum-workspace

---

#### Prerequisites

- You must have Docker engine running on your Linux, or Mac OS machine. If you use Windows, make sure to install Docker Desktop.

- You must have this project cloned or downloaded, and opened on your local machine.

_If you would like to build it from scratch and see what you'd like to customize, check the `docs/axum-leptos.md` inside `axum_workspace` directory of Git branch `axum-leptos` for this repo_

### Instructions to Run

---

First time running this project? Here are the action steps to follow:

#### Step 1.

Make sure you are inside directory `axum_workspace/`, and build Docker image with any name, or you could use name and tag - `axum_leptos_docker:1.0`, using command:

```
docker build -t axum_leptos_docker:1.0 .
```

<pre>NOTE:</pre> If you are using a custom image name, your docker image build command would look like this:

```
docker build -t <your-custom-docker-image-name-and-tag-here> .
```

#### Step 2.

Make sure you are inside directory `axum_workspace`, and then spin up docker containers specified inside `docker-compose.yml` by entering command:

```
docker-compose up
```

OR

```
docker compose up
```

#### Step 3.

Open your choice browser, and launch the app using url: `localhost:8070`

You should see a welcome page saying that "Welcome! This Rust Axum app is now Running at Port 8070"

---

### Project Maintainance (Locally)

Follow this section of this guide to rurun an updated app with code changes inside Docker.

- Take down spun-up docker containers by running command `docker-compose down` OR `docker compose down` from inside directory `axum_workspace`.

```
docker compose down
```

- Remember to delete volumes when you make changes to the source code, especially when you add new migrations. Delete the docker volume for this project `axum_leptos_docker` using command:

```
docker volume rm axum_leptos_docker_db-data
```

- Remember to delete existing Docker image builds whenever you make changes to the source code and want to make a new image with the changes. Deleting the old image would make sure that your code changes reflect inside the new docker image build. To delete existing image `axum_leptos_docker:1.0` for example, enter command:

```
docker rmi axum_leptos_docker:1.0
```

OR

```
docker image rm axum_leptos_docker:1.0
```

- Finally, after you have run commands `docker compose down`, `docker rmi axum_leptos/_docker:1.0`, and `docker volume rm axum_leptos_docker_db-data` one at a time, you can now begin the Docker image building and container spin up process again.
  Refer back to `Step 1` and `Step 2` for a reference on how to build a new docker image, and spin up new Docker containers containers respectively.

_N.B: Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`._

---

### Running your project (without Docker)

```bash
cargo leptos watch
```

### Compiling for Release

```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

### Testing Your Project

```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.

### Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
start-axum
site/
```

Set the following enviornment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="start-axum"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.
