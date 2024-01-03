mod domain;

use domain::git::Git;

fn main() {
    let git = Git::new("own-git".to_string());
    println!("{}", git.echo());
}
