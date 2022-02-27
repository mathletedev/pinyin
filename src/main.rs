use std::error::Error;

use reqwest::blocking::get;
use serde_json::{from_str, Value};

fn main() -> Result<(), Box<dyn Error>> {
	let res: Value = from_str(get("https://www.google.com/inputtools/request?ime=pinyin&ie=utf-8&oe=utf-8&app=translate&num=10&text=nihao")?.text()?.as_str())?;

	for s in res[1][0][1].as_array().unwrap() {
		println!("{}", s.as_str().unwrap());
	}

	Ok(())
}
