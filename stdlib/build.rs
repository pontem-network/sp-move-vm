use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

const PONT_STDLIB_DIR_NAME: &str = "pont-stdlib";
const PONT_REPO: &str = "https://github.com/pontem-network/pont-stdlib.git";
const PONT_STDLIB_REV: &str = "52d6f3b92f46f0333b0efff732d96ad129edbac0";

const MOVE_STDLIB_DIR_NAME: &str = "move-stdlib";
const MOVE_REPO: &str = "https://github.com/pontem-network/move-stdlib.git";
const MOVE_STDLIB_REV: &str = "deb7a9e8a33a675239200940f2c87b31d727025b";

fn main() {
    clone_and_build(PONT_STDLIB_DIR_NAME, PONT_REPO, PONT_STDLIB_REV);
    clone_and_build(MOVE_STDLIB_DIR_NAME, MOVE_REPO, MOVE_STDLIB_REV);
}

fn clone_and_build(dir: &str, repo: &str, rev: &str) {
    let path: &Path = dir.as_ref();
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
    run(".", "git", &["clone", repo]);
    run(dir, "git", &["checkout", rev]);
    run(dir, "dove", &["build", "-p"]);
}

pub fn run(path: &str, cmd: &str, args: &[&str]) {
    let status = Command::new(cmd)
        .current_dir(path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if !status.success() {
        panic!("Failed to run {} {} {:?}", path, cmd, args);
    }
}
