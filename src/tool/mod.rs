mod chroot;
mod mount;
mod qemu;

pub use chroot::chroot;
pub use mount::mount;
pub use qemu::qemu;

use crate::error::*;
use failure::ResultExt;
use std::path::PathBuf;
use std::process::Command;
use which::which;

#[derive(Debug)]
pub struct Tool {
    exec: PathBuf,
}

impl Tool {
    pub fn find(name: &'static str) -> Result<Self, Error> {
        Ok(Self {
            exec: which(name).context(ErrorKind::NoTool(name))?,
        })
    }

    pub fn execute(&self) -> Command {
        Command::new(&self.exec)
    }
}
