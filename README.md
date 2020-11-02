[![Build status on GitLab CI][gitlab-ci-master-badge]][gitlab-ci-link]
[![Newest release on crates.io][crate-version-badge]][crate-link]
[![Documentation][docs-badge]][docs]
[![Project license][crate-license-badge]](LICENSE)

[crate-license-badge]: https://img.shields.io/crates/l/git-state.svg
[crate-link]: https://crates.io/crates/git-state
[crate-version-badge]: https://img.shields.io/crates/v/git-state.svg
[docs-badge]: https://docs.rs/git-state/badge.svg
[docs]: https://docs.rs/git-state
[gitlab-ci-link]: https://gitlab.com/timvisee/git-state/pipelines
[gitlab-ci-master-badge]: https://gitlab.com/timvisee/git-state/badges/master/pipeline.svg

# git-state
A simple binary and Rust library to probe the state of a git repository.
Useful for shell prompts.

This reimplements [`git2::git_repository_state`][git2-function] in pure Rust.
This doesn't have any nasty compile or runtime `git2` dependencies.
The git command-line interface doesn't provide this functionality.

## States
Any of the following state is returned:

```
Clean
Merge
Revert
RevertSequence
CherryPick
CherryPickSequence
Bisect
Rebase
RebaseInteractive
RebaseMerge
ApplyMailbox
ApplyMailboxOrRebase
```

## Usage
Command-line usage:

```bash
cd my-repository/
git-state

# or
git-state my-repository/
```

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.

[git2-function]: https://libgit2.org/libgit2/#HEAD/group/repository/git_repository_state
