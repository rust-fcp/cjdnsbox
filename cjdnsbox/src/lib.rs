extern crate mktemp;

use std::convert::AsRef;
use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use mktemp::Temp;

mod common;

pub use common::Controller;

pub fn run<CmdPath: AsRef<OsStr>, HookPath: AsRef<Path>>(command_path: CmdPath, config: &str, hook_lib_path: HookPath) -> io::Result<Controller> {
    let dir = Temp::new_dir()?;
    let mut child = Command::new(command_path)
        .arg("--nobg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("LD_PRELOAD", hook_lib_path.as_ref())
        .env("CJDNSBOX_SOCKETS_DIR", dir.as_ref())
        .spawn()
        ?;
    child.stdin.as_mut().unwrap().write(config.as_bytes())?;
    Ok(Controller { process: child })
}
