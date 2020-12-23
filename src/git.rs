use anyhow::Result;
use git2::{Cred, Repository, RemoteCallbacks};
use std::{
  env,
  path::PathBuf,
};

pub(crate) fn clone(
    repo: &str, 
    target_dir: &PathBuf, 
    needs_password: bool
) -> Result<()> {
  
}
