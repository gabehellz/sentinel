# Contributing

Thanks for your interest in improving this project!

## How To Contribute

There are many ways that you can contribute to this project. Some of these are:

1. **Opening an issue:** you can suggest the implementation of a new feature, command, report a bug or enhancement of something;
2. **Adding context:** you can provide additional context to existing issues to help solve issues;
3. **Resolving issues:** you can open a Pull Request that solves an issue.

### Use issues

We use issues to plan things that need to be done. Before creating a pull request for creating new features, enhancing something or fixing a bug, please create a issue and describe them.

#### How to use issues

1. Go to the [issues tab](https://github.com/gabehellz/sentinel/issues);
2. Click the "New Issue" button;
3. Select the issue template according to what you want to contribute;
4. Answer the questions and provide details;
5. Add a title in front of the title placeholder.

### Commit Conventions

Commit conventions and pattern we follow for this project:

- **fix:** a commit that patches a bug or fix something code-related;
- **feat:** a commit implementing a new feature;
- **chore:** a commit changing something that doesn't affect code;
- **refactor:** a commit refactoring some code;
- **ci:** a commit related to GitHub Actions or CI.

Examples:

```
feat(ban): implementing ban command (#45)
```

```
feat: implementing kick command (#46)
```

```
fix(ban): user objects were not properly checked (#45)
```

You can add more context to the commit referencing the issue (`(#45)` in the end of the message) or an identifier after the commit type (`(ban)` in `feat(ban)`, for example).

### Use Pull Requests

Please, don't commit directly into `master` branch. Create a fork of this repository or a new branch (if you are a contributor) and Pull Request your changes into the `master` branch.

Don't forget to check out the **Implementation/Review Check-list**.

#### How to use Pull Requests

1. Go to the [pull requests tab](https://github.com/gabehellz/sentinel/pulls);
2. Click the "New pull request" button;
3. Provide details according to the template;
4. Add a title in your pull request;
5. Add @gabehellz as reviewer of the pull request.

### Implementation/Review Check-list

Things to check when creating or reviewing code:

- [ ] **Compilation:** the project compiles without any warnings or errors
- [ ] **Dependencies:** `cargo deny check` advisories, bans, licenses and sources are marked as `ok`
- [ ] **Linting 1:** `cargo fmt --all -- --check` doesn't return any warnings or errors
- [ ] **Linting 2:** `cargo clippy --verbose -- -D warnings` doesn't return any warnings or errors
- [ ] **Organization:** code is organized into properly modules (e.g., commands inside `commands` module)
