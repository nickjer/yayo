use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct DataFile {
  path: PathBuf,
}

impl DataFile {
  pub fn new(application: &str) -> Result<DataFile> {
    let proj_dirs = ProjectDirs::from("org", "", application).context(
      "No valid home directory could be retrieved from the operating system",
    )?;
    let path = proj_dirs.data_local_dir();
    fs::create_dir_all(path).with_context(|| {
      format!("Failed to create directory {}", path.display())
    })?;
    Ok(DataFile {
      path: path.join("data.json"),
    })
  }

  pub fn read(&self) -> Result<Option<String>> {
    if self.path.is_file() {
      Ok(Some(fs::read_to_string(&self.path).with_context(|| {
        format!("Failed to read from {}", self.path.display())
      })?))
    } else {
      Ok(None)
    }
  }

  pub fn write(&self, data: &str) -> Result<()> {
    fs::write(&self.path, data)
      .with_context(|| format!("Failed to write to {}", self.path.display()))
  }
}
