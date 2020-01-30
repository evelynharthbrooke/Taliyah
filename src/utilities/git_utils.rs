use git2::{ErrorCode, Repository};

/// Retrieves the current git branch in a git repository.
pub fn show_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
        Err(e) => return format!("An error occured: {:?}", e),
    };

    let head = head.as_ref().and_then(|h| h.shorthand());
    head.unwrap().to_string()
}

/// Retrieves the latest HEAD revision for the current git repository.
pub fn show_head_rev(repo: &Repository) -> String {
    let revspec = repo.revparse("HEAD").unwrap();
    let revision = revspec.from().unwrap();
    revision.short_id().unwrap().as_str().unwrap().to_string()
}
