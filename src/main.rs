use std::{env::args, error::Error};

use clipboard_ext::{prelude::ClipboardProvider, x11_fork::ClipboardContext};
use colored::Colorize;
use reqwest::blocking::get;
use serde_json::{from_str, Value};

fn main() -> Result<(), Box<dyn Error>> {
	let args = args()
		.collect::<Vec<String>>()
		.splice(1.., [])
		.collect::<Vec<String>>();

	let res = from_str::<Value>(get(format!("https://www.google.com/inputtools/request?ime=pinyin&ie=utf-8&oe=utf-8&app=translate&num=10&text={}", args.join(" ")))?.text()?.as_str())?;

	ClipboardContext::new()
		.unwrap()
		.set_contents(res[1][0][1][0].as_str().unwrap().to_string().to_owned())?;

	for (i, s) in res[1][0][1].as_array().unwrap().iter().enumerate() {
		println!(
			"{} {} {}",
			res[1][0][3]["annotation"][i].as_str().unwrap().blue(),
			"=>".green(),
			s.as_str().unwrap()
		);
	}

	Ok(())
}
