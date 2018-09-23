use git;
use github_rs::client::{Executor, Github};
use github_rs::StatusCode;
use serde_json::Value;
use CONFIG;

pub fn scrape_for_events() {
    git::fetch();
    git::prune();
    git::branch(&CONFIG.mainbranch.to_string());
    let client = Github::new(&CONFIG.githubtoken).unwrap();
    let event_url = format!("repos/{}/events?per_page=100", &CONFIG.baserepo);
    println!("Scraping repo events: {}", &event_url);
    let events = client.get().custom_endpoint(&event_url).execute::<Value>();
    match events {
        Ok((_headers, _status, json)) => {
            if let Some(json) = json {
                for val in json.as_array().unwrap().iter() {
                    if val["type"] == "PullRequestEvent" {
                        if val["payload"]["pull_request"]["merged"] == true {
                            let prnum = &val["payload"]["number"].to_string().replace("\"", "");
                            println!("PR {} was merged upstream!", prnum);
                            let branch_name = format!("upstream-merge-{}", prnum);
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
                                    if status != StatusCode::Created {
                                        println!(
                                            "Opening PR mirror for {} returned status {}",
                                            prnum, status
                                        )
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
}
