mod domain;

use domain::git::Git;

fn main() {
    let mut git = Git::new(None, "own-git".to_string());
    git.echo();
    let mut commit = git.commit("hello".to_string());
    commit.echo_message();

    let mut commit = git.commit("second commit".to_string());
    commit.echo_message();
}
