# Contributing

## Reporting Issues
Feel free to open Issues for any Discussions, Bugs or Improvements. Please adhere to the provided Templates and provide the required context. 

## Pull Request Process
1. Please open an Issue describing the bug or feature you are intending to fix. If you are working on an existing Issue comment on relevant Issue in order to let other Contributors know that you are working on it

2. [Fork](https://docs.github.com/en/get-started/quickstart/fork-a-repo) the Repository using the standard workflow and set up a new branch to work in. Make sure that each group of changes is done in a separate branch, so that PRs only contain relevant changes 

3. This Project uses [RustFmt](https://github.com/rust-lang/rustfmt) to ensure proper formatting. PRs that do not adhere to the formatting will not be able to be merged! The formatting can be done using `cargo fmt --all`. If you are using IntelliJ Idea for developing you can set up RustFmt using the `Languages&Frameworks > Rust > RustFmt` settings

4. This Project uses [Clippy](https://github.com/rust-lang/rust-clippy) as a code linter. Execute it before committing using `cargo clippy -- -D warnings`. Intellij allows you to run Clippy while developing, to find linter errors as they happen

5. All significant changes should be accompanied by tests. If you are unsure about testing, take a look at existing tests. Make sure all tests run before creating a PR using `cargo test`

6. Finally, push your commits to your fork and submit a [pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request). Summarize your changes in the Description of the Pull Request