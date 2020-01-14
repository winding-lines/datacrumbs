use std::fs::File;
use std::path::Path;
use std::process::Command;

use failure::Fallible;
use git2::Repository;

pub fn status(diff_location: &Path) -> Fallible<String> {
    let repo = Repository::discover(".")?;
    let head = repo
        .head()?
        .target()
        .map(|s| s.to_string())
        .unwrap_or_default();
    let file_name = diff_location.join("datacrumbs.patch");
    let file = File::create(file_name)?;

    Command::new("git")
        .arg("diff")
        .stdout(file)
        .output()?;
    Ok(head)
}
