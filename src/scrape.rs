use db;
use git;
use github_rs::client::{Executor, Github};
use serde_json::Value;
use std::thread;
use std::time::Duration;
use CONFIG;

pub fn scrape_for_events() {
    git::fetch();
    git::branch(&CONFIG.mainbranch.to_string());
    let client = Github::new(&CONFIG.githubtoken).unwrap();
    let event_url = format!("repos/{}/events?per_page=99", &CONFIG.baserepo);
    println!("Scraping repo events: {}", &event_url);
    loop {
        for page in 1..3 {
            let events = client.get().custom_endpoint(&format!("{}&page={}", &event_url, page)).execute::<Value>();
            match events {
                Ok((_headers, _status, json)) => {
                    if let Some(json) = json {
                        for val in json.as_array().unwrap().iter() {
                            if val["type"] == "PullRequestEvent" {
                                let mut our_db = db::read_db();
                                if val["payload"]["pull_request"]["merged"] == true {
                                    let prnum =
                                        &(val["payload"]["number"]).to_string().replace("\"", "");
                                    if our_db.contains(prnum) {
                                        continue;
                                    }
                                    println!("PR {} was merged upstream!", prnum);
                                    let branch_name = format!("{}{}", &CONFIG.branchprefix, prnum);
                                    git::new_branch(&branch_name);
                                    git::cherry_pick(
                                        &val["payload"]["pull_request"]["merge_commit_sha"]
                                            .to_string()
                                            .replace("\"", ""),
                                    );
                                    git::branch(&CONFIG.mainbranch.to_string()); // go home rust you're drunk
                                    git::push_upstream(&branch_name);
                                    let post_url = format!("repos/{}/pulls", CONFIG.targetrepo);
                                    let post_data = json!({
                                    "title": format!("[MIRROR] {}", val["payload"]["pull_request"]["title"].as_str().unwrap()),
                                    "head": branch_name,
                                    "base": CONFIG.mainbranch,
                                    "body": format!("Original PR: {}\r\n--------------------\r\n{}", val["payload"]["pull_request"]["html_url"].as_str().unwrap(), val["payload"]["pull_request"]["body"].as_str().unwrap()),
                                    "maintainer_can_modify": true
                                });
                                    let resp = client
                                        .post(post_data)
                                        .custom_endpoint(&post_url)
                                        .execute::<Value>();
                                    match resp {
                                        Ok((_headers, status, _json)) => {
                                            println!(
                                                "Opening PR mirror for {} returned status {}",
                                                prnum, status
                                            );
                                            our_db.push(prnum.to_string());
                                            db::update_db(our_db);
                                            let label_url = format!(
                                                "repos/{}/issues/{}/labels",
                                                CONFIG.targetrepo, prnum
                                            );
                                            let issue_data = json!(["Upstream PR Merged"]);
                                            match client
                                                .post(issue_data)
                                                .custom_endpoint(&label_url)
                                                .execute::<Value>()
                                            {
                                                Ok(_) => (),
                                                Err(e) => println!(
                                                    "Error setting label on PR {}: {:?}",
                                                    prnum, e
                                                ),
                                            }
                                        }
                                        Err(e) => println!("{}", e),
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("{}", e),
            }
            println!("Sleeping for 60 seconds.");
            thread::sleep(Duration::new(60, 0));
        }
    }
}
