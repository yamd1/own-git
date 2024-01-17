mod domain;

use domain::git::Git;

fn main() {
    let mut git = Git::new("own-git".to_string());
    git.echo();
    git.commit("hello".to_string());
    git.commit("second commit".to_string());

    git.log();
}
