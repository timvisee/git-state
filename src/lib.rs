use std::fmt;
use std::path::Path;

/// Determine the state of a git repository.
///
/// This checks whether the progress is in a merging/rebasing/bisecing/etc state.
/// If in no such state, `Clean` is returned.
///
/// Based on:
/// - https://github.com/libgit2/libgit2/blob/52294c413100ed4930764addc69beadd82382a4c/src/repository.c#L2867-L2908
/// - https://libgit2.org/libgit2/#HEAD/type/git_repository_state_t
pub fn git_state(repo: &Path) -> Result<RepositoryState, ()> {
    let git_dir = repo.join(".git");
    let git_dir = if git_dir.is_dir() { &git_dir } else { repo };

    if git_dir.join("rebase-merge/interactive").is_file() {
        Ok(RepositoryState::RebaseInteractive)
    } else if git_dir.join("rebase-merge").is_dir() {
        Ok(RepositoryState::RebaseMerge)
    } else if git_dir.join("rebase-apply/rebasing").is_file() {
        Ok(RepositoryState::Rebase)
    } else if git_dir.join("rebase-apply/applying").is_file() {
        Ok(RepositoryState::ApplyMailbox)
    } else if git_dir.join("rebase-apply").is_dir() {
        Ok(RepositoryState::ApplyMailboxOrRebase)
    } else if git_dir.join("MERGE_HEAD").is_file() {
        Ok(RepositoryState::Merge)
    } else if git_dir.join("REVERT_HEAD").is_file() {
        if git_dir.join("sequencer/todo").is_file() {
            Ok(RepositoryState::RevertSequence)
        } else {
            Ok(RepositoryState::Revert)
        }
    } else if git_dir.join("CHERRY_PICK_HEAD").is_file() {
        if git_dir.join("sequencer/todo").is_file() {
            Ok(RepositoryState::CherryPickSequence)
        } else {
            Ok(RepositoryState::CherryPick)
        }
    } else if git_dir.join("BISECT_LOG").is_file() {
        Ok(RepositoryState::Bisect)
    } else {
        Ok(RepositoryState::Clean)
    }
}

/// Git repository state.
///
/// Based on:
/// - https://libgit2.org/libgit2/#HEAD/type/git_repository_state_t
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RepositoryState {
    Clean,
    Merge,
    Revert,
    RevertSequence,
    CherryPick,
    CherryPickSequence,
    Bisect,
    Rebase,
    RebaseInteractive,
    RebaseMerge,
    ApplyMailbox,
    ApplyMailboxOrRebase,
}

impl fmt::Display for RepositoryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
