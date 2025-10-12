# Rust 🦀

## Understanding 💡

An `!` in Rust indicates a macro, which is expanded before the code is compiled into Rust's internal representation. Without the `!`, you are calling a function, not a macro.

## Command ⌨️

### Cargo 📦

`cargo new <project-name>` -> create new project.

`cargo new <project-name> --vcs none` -> create new project without git.

`cargo build` -> build the project unoptimization and debug (if its still under developing).

`cargo build --release` -> for final build of the project to be released with optimization (production things).

`cargo run` -> build and run the project.

`cargo check` -> checking for error code without create executable file.

### Manual 🖐️

`rustc <name-file.rs>` -> compile the file.

`./<name-file>` -> running the compiled file.