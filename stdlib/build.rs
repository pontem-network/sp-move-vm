use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

const STDLIB_DIR_NAME: &'static str = "move-stdlib";
const REPO: &'static str = "https://github.com/pontem-network/move-stdlib.git";
const REV: &'static str = "4b074a83aefb14da0b0f5c22276637423dad738b";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let path: &Path = STDLIB_DIR_NAME.as_ref();
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
    run(".", "git", &["clone", REPO]);
    run(STDLIB_DIR_NAME, "git", &["checkout", REV]);
    run(STDLIB_DIR_NAME, "dove", &["build", "-p"]);
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
