use std::{
    fs,
    io::{Error, Result},
    path::PathBuf,
};

use clap::Parser;
use duct::cmd;
trait FindFile {
    fn find_git_root(&self) -> Result<PathBuf> {
        let git_root = cmd!("git", "rev-parse", "--show-toplevel").read()?;
        Ok(PathBuf::from(git_root))
    }
    #[allow(dead_code)]
    fn print_git_root(&self) {
        if let Ok(git_root) = self.find_git_root() {
            println!("{:#?}", git_root);
        } else {
            println!("not find git root");
        }
    }
    fn find_git_root_file(&self, file: &str) -> Result<Vec<PathBuf>> {
        let git_root_pwd = self.find_git_root()?;
        let path_list = self.find_file_with_extension(git_root_pwd, file)?;
        if path_list.is_empty() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                format!("not find {}", file),
            ));
        }
        Ok(path_list)
    }
    fn find_file_with_extension(&self, git_root_pwd: PathBuf, file: &str) -> Result<Vec<PathBuf>> {
        let mut path_list: Vec<PathBuf> = Vec::new();
        for entry in fs::read_dir(git_root_pwd)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                path_list.extend(self.find_file_with_extension(path, file)?);
            } else if let Some(extension) = path.extension() {
                if extension == file {
                    path_list.push(path);
                }
            }
        }
        Ok(path_list)
    }
}

struct LazyMdk {}
impl<U> FindFile for U {}
impl LazyMdk {
    fn mdk_cmd(&self, cmd: &str) -> Result<()> {
        let path_list = self.find_git_root_file("uvprojx")?;
        if let Some(parent_path) = path_list[0].parent() {
            let build = parent_path.join("build.log");
            let build_str = build.to_str();
            if let Some(build_pwd) = build_str {
                match cmd {
                    "-d" => {
                        cmd!("uv4", cmd, &path_list[0], "-o", build_pwd).run()?;
                    }
                    "-f" | "-b" => {
                        cmd!("uv4", cmd, &path_list[0], "-o", build_pwd).run()?;
                        cmd!("bat", build_pwd).run()?;
                    }
                    "" => {
                        cmd!("uv4", &path_list[0]).run()?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    /// Specify your name
    cmd: Option<String>,
}
fn main() {
    let lazy_mdk = LazyMdk {};
    lazy_mdk.print_git_root();
    let cli = Cli::parse();
    if let Some(s) = cli.cmd {
        let cmd = format!("-{}", s);
        let _ = lazy_mdk.mdk_cmd(&cmd);
    } else {
        let _ = lazy_mdk.mdk_cmd("");
    }
}
