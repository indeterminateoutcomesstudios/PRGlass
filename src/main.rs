extern crate config;
extern crate git2;
extern crate github_rs;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod git;
mod scrape;
mod settings;

use settings::Settings;

lazy_static! {
    pub static ref CONFIG: Settings = { Settings::new().unwrap() };
}

fn main() {
    print_config();
    git::fetch();
    scrape::scrape_for_events();
}

fn print_config() {
    println!("Loaded config from Config.toml!");
    println!("Base repo: {}", CONFIG.baserepo);
    println!("Target repo: {}", CONFIG.targetrepo);
    println!("Repo path: {}", CONFIG.repoloc);
}
