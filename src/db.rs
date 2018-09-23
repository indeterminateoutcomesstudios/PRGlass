use serde_json::{from_str, to_writer};
use std::fs::File;
use std::io::Read;

pub fn read_db() -> Vec<String> {
	let mut file = File::open("merged_prs.json").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let our_vec: Vec<String> = from_str(&data).unwrap();
	our_vec
}

pub fn update_db(data: Vec<String>) {
	let file = File::create("merged_prs.json");
	match file {
		Ok(mut f) => {
			let res = to_writer(&mut f, &data);
			match res {
				Ok(_) => (),
				Err(e) => println!("Error saving merged_prs.json: {:?}", e),
			}
		}
		Err(e) => println!("Error opening merged_prs.json: {:?}", e),
	}
}
