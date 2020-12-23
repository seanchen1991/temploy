use anyhow::Result;
use dialoguer::Password;
use git2::{Cred, Repository, RemoteCallbacks};
use std::{
    env,
    path::{Path, PathBuf},
};

pub(crate) fn clone(
    repo: &str, 
    target_dir: &PathBuf, 
    needs_password: bool
) -> Result<()> {
    if repo.contains("http") {
        Repository::clone(repo, &target_dir)?;
    } else {
        let password = match needs_password {
            true => {
                Password::new()
                    .with_prompt("Enter password for .ssh/rsa_id")
                    .interact()?
                    .into()
            },
            false => None,
        };

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_url, username, _allowed| {
            Cred::ssh_key(
                username.unwrap(),
                None,
                Path::new(&format!(
                    "{}/.ssh/id_rsa",
                    env::var("HOME").expect("Cannot fetch $HOME")
                )),
                password.as_deref(),
            )
        });
    }

    Ok(())
}
