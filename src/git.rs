use git2::Repository;
use std::process::Command;
use CONFIG;

pub fn fetch() {
	let repo = Repository::open(&CONFIG.repoloc).unwrap();
	let mut remote = repo.find_remote("upstream").unwrap();
	remote
		.fetch(&["refs/heads/*:refs/heads/*"], None, None)
		.unwrap();
}

pub fn cherry_pick(sha: &String) {
	let cherry_bomb = Command::new("git")
		.current_dir(&CONFIG.repoloc)
		.arg("cherry-pick")
		.arg("--no-gpg-sign")
		.arg("--no-edit")
		.arg(sha)
		.output()
		.expect(&format!("Failed to cherry-pick commit {}", sha));
	if String::from_utf8_lossy(&cherry_bomb.stderr).contains("is a merge but no -m option") { // this does exactly what you think it does
		Command::new("git")
			.current_dir(&CONFIG.repoloc)
			.arg("cherry-pick")
			.arg("--no-gpg-sign")
			.arg("--no-edit")
			.arg("-m 1")
			.arg(sha)
			.output()
			.expect(&format!("Failed to cherry-pick merge commit {}", sha));
	}
	Command::new("git")
		.current_dir(&CONFIG.repoloc)
		.arg("add")
		.arg("-A")
		.output()
		.expect(&format!("Failed to add all on commit {}", sha));
	Command::new("git")
		.current_dir(&CONFIG.repoloc)
		.arg("cherry-pick")
		.arg("--no-gpg-sign")
		.arg("--no-edit")
		.arg("--continue")
		.output()
		.expect(&format!("Failed to continue cherry-pick on commit {}", sha));
}

pub fn new_branch(name: &String) {
	Command::new("git")
		.current_dir(&CONFIG.repoloc)
		.arg("checkout")
		.arg("-b")
		.arg(name)
		.output()
		.expect(&format!("Failed to make new branch '{}'", name));
}

pub fn branch(name: &String) {
	fetch();
	Command::new("git")
		.current_dir(&CONFIG.repoloc)
		.arg("checkout")
		.arg(name)
		.output()
		.expect(&format!("Failed to make new branch '{}'", name));
	Command::new("git")
		.current_dir(&CONFIG.repoloc)
		.arg("reset")
		.arg("--hard")
		.arg(format!("origin/{}", &CONFIG.mainbranch))
		.output()
		.expect(&format!("Failed to reset branch '{}'", name));
}

pub fn push_upstream(name: &String) {
	Command::new("git")
		.current_dir(&CONFIG.repoloc)
		.arg("push")
		.arg("-u")
		.arg("origin")
		.arg(name)
		.output()
		.expect(&format!("Failed to push branch '{}'", name));
}

pub fn prune() {
	Command::new("git")
		.current_dir(&CONFIG.repoloc)
		.arg("remote")
		.arg("prune")
		.arg("origin")
		.output()
		.expect(&"Failed to prune origin!");
}
