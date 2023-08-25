** Still Under Construction **

## Start from scratch here

You may follow these steps:

1. Make sure `cargo-leptos` is installed on your local dev machine. If unsure, still run the command to install it - the installation command would either refuse to install or update to a new version if your already have `cargo-leptos` installed, but run the command to install it. The command to install it is:

```sh
cargo install cargo-leptos
```

2. Clone a template workspace from the <code>Leptos</code> repo on GitHub with command:

```sh
cargo leptos new --git https://github.com/Gentle/start-axum-workspace
```

OR

```sh
cargo leptos new --git https://github.com/leptos-rs/start-axum-workspace
```

You'll be asked to provide a name for the project. In this repo's case, it was named `start-axum-workspace`. Feel free to provide a different name, it doesn't matter since it will only exist temporarily.

3. This step may not be applicable to you.

Since cloning the `start-axum-workspace` project (https://github.com/Gentle/start-axum-workspace) may essentially create a new Git repo inside an already exiting repo in the future, You may have to run a Git repo removal command inside the newly created sub-workspace `start-axum-workspace` just in-case, using command:

```sh
cd start-axum-workspace && rm -rf .git
```

4. Since the newly created `start-axum-workspace` cargo leptos template is itself a workspace which now resides inside a prior workspace `axum_workspace`, you'll have to clean-up the parent-workspace `axum_workspace` by copying (actually moving) all of the contents inside sub-workspace `start-axum-workspace` directly into `axum_workspace` at its root-dir level, and then delete the emptied-out `start-axum-workspace` directory. The consecutive commands used were:

```sh
cd start-axum-workspace
```

```sh
mv * ..
```

- Inside `axum_workspace`, delete `target`

```sh
rm -rf target
```

- Inside the `axum_workspace` level `Cargo.toml` change parameter:

```toml
[[workspace.metadata.leptos]]
# this name is used for the wasm, js and css file names
name = "start-axum-workspace"
```

..to

```toml
[[workspace.metadata.leptos]]
# this name is used for the wasm, js and css file names
name = "axum_workspace"
```

- Manually move `.gitgnore` away from `start-axum-workspace` since it'll persist inside `start-axum-workspace` during the `mv` operation into `axum_workspace` root-dir level. Replace any prior `.gitignore` file.

- Add new file location - /axum_postgres_docker/.env to `.gitignore`

- Delete the now empty directory `start-axum-workspace`.

- Change directory to `axum_workspace`'s root-dir, opened `Cargo.toml` inside it, and created a new exclude list inside `axum_workspace`'s `Cargo.toml`. The exclude list comprise directories - `axum_postgres_docker`, and `docs`.

5. Create a `rust-toolchain.toml` file inside `axum_workspace`'s root-dir level to specify that the rust `nightly` toolchain be used to compile the entire workspace. Some leptos code extras require it.

```rust-toolchain.toml
[toolchain]
channel = "nightly"
targets = ["wasm32-unknown-unknown"]
```

6. Exclude `docs` and `axum_postgres_docker` if it still exists inside your `axum_workspace`. Add the following to your `axum_workspace` top of Cargo.toml right below a newline after `members = ["app", "frontend", "server"]`:

```toml
exclude = [
    "docs",
    "axum_postgres_docker",
]
```

7. Edit `axum_workspace/.github/workflows/rust.yml`, such that events 'on.push' and 'on.pull_request' now target Git branch `start-axum-workspace`.

8. Optionally make changes inside the `Cargo.toml` of corresponsing `app`, `frontend`, and `server` member-crates of `axum_workspace`. You'll find suggestions as to what these changes are.

9. Run `cargo build` anywhere inside in the `axum_worspace` directory.

Note: At this stage, you might encounter a compilation error that looks similar to this -

```
...(head, tail) = html_parts_separated(options, use_context::<MetaC...
| ^^^^^^^^^^^^^^^^^^^^ ------- an argument of type `leptos::Scope` is missing
```

If you follow the link to the module causing the build error, you'll see the following faulty `html_parts_seperated()` function call in line `647` -

```rs
let (head, tail) = html_parts_separated(options, use_context::<MetaContext>(cx).as_ref());
```

Fix the error.

<h6>The Fix</h6>:

The simple fix to this error as suggested by the Rust compiler itself is to add a new argument of type `leptos::Scope` as the first argument to `html_parts_separated()` function call, at line `647`.

The fix should look as follows:

```rs
let (head, tail) = html_parts_separated(cx, options, use_context::<MetaContext>(cx).as_ref());
```

10. Run command to launch leptos app at the `axum_workspace` root-level dir:

```sh
cargo leptos watch
```

It may take a while to compile. You may want to take a 15 mins break.

11. Once cargo leptos is down compiling your entire workspace, open your favorite browser and check-out url `localhost:3000`.
