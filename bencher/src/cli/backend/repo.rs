use std::fs;
use std::path::Path;

use git2::Direction;
use git2::{Commit, Cred, ObjectType, RemoteCallbacks};
use git2::{Oid, Remote, Repository, Signature};
use reports::{Report, Reports};
use tempfile::tempdir;

use crate::cli::clap::CliPush;
use crate::cli::clap::CliRepo;
use crate::BencherError;

const BENCHER_DIR: &str = "bencherdb";
const BENCHER_FILE: &str = "bencher.json";
const BENCHER_MESSAGE: &str = "bencher";
const DEFAULT_BRANCH: &str = "master";

#[derive(Debug)]
pub struct Repo {
    url: String,
    key: Option<String>,
    branch: Option<String>,
    push: Option<Push>,
}

#[derive(Debug)]
pub struct Push {
    name: Option<String>,
    email: Option<String>,
    message: Option<String>,
}

impl From<CliRepo> for Repo {
    fn from(repo: CliRepo) -> Self {
        Self {
            url: repo.url,
            key: repo.key,
            branch: repo.branch,
            push: repo.push.push.then(|| Push::from(repo.push)),
        }
    }
}

impl From<CliPush> for Push {
    fn from(update: CliPush) -> Self {
        Self {
            name: update.name,
            email: update.email,
            message: update.message,
        }
    }
}

impl Repo {
    pub fn save(&self, report: Report) -> Result<String, BencherError> {
        let temp_dir = tempdir()?;
        let bencher_dir = temp_dir.path().join(BENCHER_DIR);
        let repo = self.clone(&bencher_dir)?;

        let bencher_file = Path::new(BENCHER_FILE);
        let bencher_path = bencher_dir.join(&bencher_file);
        let reports = Self::update(&bencher_path, report)?;
        if let Some(push) = &self.push {
            push.save(
                &repo,
                &bencher_file,
                &bencher_path,
                &reports,
                self.key.as_deref(),
                self.branch.as_deref(),
            )?;
        }
        Ok(reports)
    }

    fn clone(&self, into: &Path) -> Result<Repository, git2::Error> {
        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks(self.key.as_deref(), false));

        // Prepare builder.
        let mut repo_builder = git2::build::RepoBuilder::new();
        let builder = if let Some(branch) = &self.branch {
            repo_builder.branch(branch)
        } else {
            &mut repo_builder
        };
        builder.fetch_options(fo);

        // Clone the project.
        builder.clone(&self.url, into)
    }

    fn update(path: &Path, report: Report) -> Result<String, serde_json::Error> {
        let mut reports = Self::load_reports(path);
        reports.add(report);
        serde_json::to_string(&reports)
    }

    fn load_reports(path: &Path) -> Reports {
        if let Ok(contents) = fs::read(path) {
            let contents = String::from_utf8_lossy(&contents);
            if let Ok(reports) = serde_json::from_str(&contents) {
                return reports;
            }
        }
        Reports::new()
    }
}

impl Push {
    pub fn save(
        &self,
        repo: &Repository,
        bencher_file: &Path,
        bencher_path: &Path,
        reports: &str,
        key: Option<&str>,
        branch: Option<&str>,
    ) -> Result<(), BencherError> {
        fs::write(bencher_path, reports)?;
        let oid = Self::add(repo, bencher_file)?;
        let commit = self.commit(repo, oid)?;
        println!("Commit added {commit}");
        self.push(key, branch, repo).map_err(|e| e.into())
    }

    fn add(repo: &Repository, path: &Path) -> Result<Oid, git2::Error> {
        let mut index = repo.index()?;
        index.add_path(path)?;
        index.write_tree()
    }

    // https://zsiciarz.github.io/24daysofrust/book/vol2/day16.html
    fn commit(&self, repo: &Repository, oid: Oid) -> Result<Oid, git2::Error> {
        let signature = self.signature(repo)?;
        let message = self.message.as_deref().unwrap_or(BENCHER_MESSAGE);
        let tree = repo.find_tree(oid)?;
        let parent_commit = Self::last_commit(repo);
        let parents = if let Ok(parent) = &parent_commit {
            vec![parent]
        } else {
            Vec::new()
        };
        repo.commit(
            Some("HEAD"), //  point HEAD to our new commit
            &signature,   // author
            &signature,   // committer
            message,      // commit message
            &tree,        // tree
            &parents,     // parents
        )
    }

    fn push(
        &self,
        key: Option<&str>,
        branch: Option<&str>,
        repo: &Repository,
    ) -> Result<(), git2::Error> {
        // Connect remote.
        let mut remote = repo.find_remote("origin")?;
        remote.connect_auth(Direction::Push, Some(callbacks(key, false)), None)?;

        // Get the branch for the refspec
        let branch = Self::branch(branch, &remote);
        let refspec: [&str; 1] = [&format!("refs/heads/{branch}:refs/heads/{branch}")];

        // Prepare push options for the callback
        let mut po = git2::PushOptions::new();
        po.remote_callbacks(callbacks(key, true));

        // Push remote.
        remote.push(&refspec, Some(&mut po)).map_err(|e| e.into())
    }

    fn signature(&self, repo: &Repository) -> Result<Signature, git2::Error> {
        if let Some(name) = &self.name {
            if let Some(email) = &self.email {
                if let Ok(signature) = Signature::now(name, email) {
                    return Ok(signature);
                }
            }
        }
        repo.signature()
    }

    fn branch(branch: Option<&str>, remote: &Remote) -> String {
        if let Some(branch) = branch {
            return branch.into();
        }

        if let Ok(buf) = remote.default_branch() {
            if let Some(branch) = buf.as_str() {
                return branch.into();
            }
        } else {
            println!("WARNING: Failed to get remote branch name");
        }

        DEFAULT_BRANCH.into()
    }

    fn last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
        let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
        obj.into_commit()
            .map_err(|_| git2::Error::from_str("Couldn't find last commit"))
    }
}

fn callbacks(key: Option<&str>, update_reference: bool) -> RemoteCallbacks<'_> {
    let mut callbacks = if let Some(key) = key {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(username_from_url.unwrap(), None, Path::new(key), None)
        });
        callbacks
    } else {
        RemoteCallbacks::new()
    };
    if update_reference {
        callbacks.push_update_reference(move |name, status| {
            if let Some(status) = status {
                Err(git2::Error::from_str(&format!(
                    "Update reference failed: {status}"
                )))
            } else {
                println!("Push commit for refspec {name} succeeded");
                Ok(())
            }
        });
    }
    callbacks
}
