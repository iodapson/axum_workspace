## Brief Description

You are inside the `scratch-pad` branch of this repo - `axum_workspace`. This branch is just aspace to experiment with a mix ideas that would be appropriately filtered and refined into appropriate other branches inside this repo.

Check `/docs/from-scratch.md` for a detailed code-walkthrough of how to get up to speed with the creation of this project (branch `scratch-pad` version of it).

This project would piggyback on an already existing project, 'realworld-axum-sqlx'. You can find the project here: https://github.com/davidpdrsn/realworld-axum-sqlx/

<pre>If you are looking for a bare-bone template version, one that simply creates a dockerized axum web-api with a Postgres docker container, and contains code to start creating route immediately, then check out the `template` branch of this project-repo.</pre>

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
