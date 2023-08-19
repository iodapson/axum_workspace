## Introduction And Description

You are inside branch `template`, the default branch of this repo - `axum_workspace`. This branch is the template version of this repo. It has a bare-bone setup for an axum web api comprising of a single postgres container, and a single containerized axum web-api. The axum api itself (`axum_postgres_docker`) does no more than provide configurations to connect to the database, and then run at an exposed port - 8090 with a single `get_root_path` route for an HTTP-GET request to path `/`, as in, `http://localhost:8090`. It also includes a starter migration named - `0001_setup.sql` inside `axum_postgres_docker/migrations`.

Check `axum_workspace/docs/template.md` for a detailed code-walkthrough of how to recreate this project (this very branch - `template` version of it) from scratch.

If you are interested in a rather full-stack guide/application-demo, a new branch `axum-leptos-demo` would soon be created to soothe that curiosity.

This repo takes inspiration from another axum project repo - `realworld-axum-sqlx`, in order to closely follow realworld Rust web-api conventions. The `realworld-axum-sqlx` repo version that this repo follows is hosted at: https://github.com/davidpdrsn/realworld-axum-sqlx/

---

### Instructions to Run

#### Prerequisites

- You must have Docker engine running on your Linux, or Mac OS machine. If you use Windows, make sure to install Docker Desktop.

- Download or clone this project.

---

First time running this project? Here are the action steps to follow:

#### Step 1

It is assumed that you have cloned or downloaded this project and is opened locally on your machine.

Make sure your inside directory `axum_workspace/axum_postgres_docker`, and copy the contents of `.env.sample` into a `.env` file using command:
Make sure you are inside directory `axum_workspace/axum_postgres_docker`, and copy the contents of `.env.sample` into a `.env` file using command:

```
cp ./.env.sample ./.env
```

#### Step 2.

Make sure you are inside directory `axum_workspace/axum_postgres_docker`, and build Docker image with any name, or you could use name and tag - `axum_postgres_docker:1.0`, using command:

```
docker build -t axum_postgres_docker:1.0 .
```

<pre>NOTE:</pre> If you are using a custom image name, your docker image build command would look like this:

```
docker build -t <your-custom-docker-image-name-and-tag-here> .
```

#### Step 3.

Make sure you are inside directory `axum_workspace/axum_postgres_docker`, and then spin up docker containers specified inside `docker-compose.yml` by entering command:

```
docker-compose up
```

OR

```
docker compose up
```

#### Step 4.

Open your choice browser, and launch the app using url: `localhost:8090`

You should see a welcome page saying that "Welcome! This Rust Axum app is now Running at Port 8090"

---

### Project Maintainance (Locally)

Follow this section of this guide to rurun an updated app with code changes inside Docker.

- Take down spun-up docker containers by running command `docker-compose down` OR `docker compose down` from inside directory `axum_workspace/axum_postgres_docker`.

```
docker compose down
```

- Remember to delete volumes when you make changes to the source code, especially when you add new migrations. Delete the docker volume for this project `axum_postgres_docker` using command:

```
docker volume rm axum_postgres_docker_db-data
```

- Remember to delete existing Docker image builds whenever you make changes to the source code and want to make a new image with the changes. Deleting the old image would make sure that your code changes reflect inside the new docker image build. To delete existing image `axum_postgres_docker:1.0` for example, enter command:

```
docker rmi axum_postgres_docker:1.0
```

OR

```
docker image rm axum_postgres_docker:1.0
```

- Finally, after you have run commands `docker compose down`, `docker rmi axum_postgres_docker:1.0`, and `docker volume rm axum_postgres_docker_db-data` one at a time, you can now begin the image building and container spin up process again.
  Refer back to `Step 2` and `Step 3` for a reference on how to build a new docker image, and spin up new Docker containers containers respectively.
