use anyhow::Result;
use git2::{Cred, RemoteCallbacks, Repository};
use std::{
    env,
    path::{Path, PathBuf},
};

pub(crate) fn clone(repo: &str, target_dir: &PathBuf) -> Result<()> {
    if repo.contains("http") {
        Repository::clone(repo, &target_dir)?;
    } else {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_url, username, _allowed| {
            Cred::ssh_key(
                username.unwrap(),
                None,
                Path::new(&format!(
                    "{}/.ssh/id_rsa",
                    env::var("HOME").expect("Cannot fetch $HOME")
                )),
                None,
            )
        });

        let mut foptions = git2::FetchOptions::new();
        foptions.remote_callbacks(callbacks);

        let mut repo_builder = git2::build::RepoBuilder::new();
        repo_builder.fetch_options(foptions);

        repo_builder.clone(repo, &target_dir)?;
    }

    Ok(())
}
