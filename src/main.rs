use std::{io::Result, path::PathBuf};

use duct::cmd;
trait FindFile {
    fn find_git_root(&self) -> Result<PathBuf> {
        let git_root = cmd!("git", "rev-parse", "--show-toplevel").read()?;
        Ok(PathBuf::from(git_root))
    }
    fn print_git_root(&self) {
        if let Ok(git_root) = self.find_git_root() {
            println!("{:#?}", git_root);
        } else {
            println!("not find git root");
        }
    }
}
struct LazyMdk {}
impl<U> FindFile for U {}
fn main() {
    let lazy_mdk = LazyMdk {};
    lazy_mdk.print_git_root();
}
