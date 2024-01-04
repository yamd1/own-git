mod domain;

use domain::git::Git;

fn main() {
    let mut git = Git::new("own-git".to_string());
    git.echo();
    let commit = git.commit("hello".to_string());
    commit.echo_message();

    let commit = git.commit("second commit".to_string());
    commit.echo_message();
}
