use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

const DIEM_STDLIB_DIR_NAME: &str = "diem-stdlib";
const DIEM_REPO: &str = "https://github.com/pontem-network/diem-stdlib.git";
const DIEM_STDLIB_REV: &str = "11609d0797dcedf6fac39f6d99ffaab1c40342da";

const MOVE_STDLIB_DIR_NAME: &str = "move-stdlib";
const MOVE_REPO: &str = "https://github.com/pontem-network/move-stdlib.git";
const MOVE_STDLIB_REV: &str = "c1306f32a3c61059e98279ad4702132eca34d848";


fn main() {
    clone_and_build(DIEM_STDLIB_DIR_NAME, DIEM_REPO, DIEM_STDLIB_REV);
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
