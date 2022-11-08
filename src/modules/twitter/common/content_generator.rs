use super::{map::map_changed_file, mock::titles};
use crate::modules::git::utils::Branch;
use crate::modules::git::Commit;
use crate::modules::git::Repo;
use log::info;
use rusttype::Font;
use rusttype::Scale;
use std::fs;
use std::include_bytes;
use std::io;
use std::io::Cursor;
use std::path::Path;
use std::str;

pub fn tweet_generator(
    data: (Vec<Commit>, Repo),
    branch: String,
) -> String {
    let commits = data.0;
    let repo = data.1;
    let title = title(&commits);
    let git_stats = git_stats(&commits, branch);
    let current_working_on = current_working_on(repo);
    let tags = tags();
    format!(
        "{}\n{}\n{}\n{}",
        title, git_stats, current_working_on, tags
    )
}

pub fn title(commits: &Vec<Commit>) -> String {
    let mut total_changed_file: u64 = 0;

    for commit in commits {
        total_changed_file += commit.changed_files;
    }

    let level = map_changed_file(total_changed_file);
    titles().get(&level).unwrap().to_string()
}

pub fn git_stats(commits: &Vec<Commit>, branch: String) -> String {
    let number_commits = commits.len();
    let changed_files = changed_files(commits);
    format!("{} commits created and total {} files changed on {} branch 👏👏", number_commits, changed_files, branch)
}

pub fn changed_files(commits: &Vec<Commit>) -> u64 {
    let mut files = 0;
    for commit in commits {
        files += commit.changed_files;
    }
    files
}

pub fn current_working_on(repo: Repo) -> String {
    format!("👉 My repo: {}\nIf anyone's interested, feel free to open issues, create PRs to contribute 💌", repo.url)
}

pub fn tags() -> String {
    "#100DaysOfCode #rust".to_string()
}
