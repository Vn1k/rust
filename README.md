# Rust 🦀

## Understanding 💡 (more detail in every rust file in the [projects](https://github.com/Vn1k/rust/blob/15716702a23c8c247508f60fe8fbdf98a82a4e7e/projects) folder)

An `!` in Rust indicates a macro, which is expanded before the code is compiled into Rust's internal representation. Without the `!`, you are calling a function, not a macro.

The Rust line `use rand::Rng;`, which follows the pattern use `<crate>::<trait>`, can be understood like TypeScript's `import { function_or_interface } from 'package-name'`

In Rust, it's possible to redefine a variable with the same name (like guess); this is called **shadowing**. It is commonly used to convert a variable from one type to another.

`Pattern => Expression/code` <- This is arm pattern

## Command ⌨️

### Cargo 📦

`cargo new <project-name>` -> create new project.

`cargo new <project-name> --vcs=git` -> create new project with git (by default no git).

`cargo build` -> build the project unoptimization and debug (if its still under developing).

`cargo build --release` -> for final build of the project to be released with optimization (production things).

`cargo run` -> build and run the project.

`cargo check` -> checking for error code without create executable file.

`cargo update` -> will ignore cargo.lock to figure all lastest version that fit for the project in cargo.toml.
And then, overwrite the version of cargo.lock.

### Manual 🖐️

`rustc <name-file.rs>` -> compile the file.

`./<name-file>` -> running the compiled file.