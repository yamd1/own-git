mod domain;

use domain::git::Git;

fn main() {
    let git = Git::new("own-git".to_string());
    println!("{}", git.echo());
    let commit = git.commit("hello".to_string());
    println!("{}", commit.echo_message());
}
