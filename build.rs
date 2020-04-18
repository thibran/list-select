use std::fs::File;
use std::io::Write;

fn main() {
    let mut f = File::create("src/const_fn.rs").unwrap();

    write!(
        f,
        "/// `rustc_version` string.
pub const RUSTC_VERSION: &str = \"{}\";",
        rustc_version()
    )
    .unwrap();

    write!(
        f,
        "\n
/// returns the first 7 git-hash characters of the current branch.
pub const GIT_CURRENT_HASH: &str = \"{}\";",
        git_current_hash(7)
    )
    .unwrap();
}

/// rustc_version string.
pub fn rustc_version() -> String {
    use std::process::Command;
    Command::new("rustc")
        .args(&["--version"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_owned())
        .ok()
        .and_then(|s| s.split(' ').nth(1).map(str::to_owned))
        .unwrap_or("--".to_owned())
}

/// returns the first N git-hash characters of the current branch.
pub fn git_current_hash<T>(opt_num: T) -> String
where
    T: Into<Option<usize>>,
{
    use std::process::Command;
    Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .map(|out| match opt_num.into() {
            Some(n) if n > 0 => {
                let mut s =
                    String::from_utf8_lossy(&out.stdout).trim().to_owned();
                s.truncate(n);
                s
            }
            _ => String::from_utf8_lossy(&out.stdout).trim().to_owned(),
        })
        .unwrap()
}
