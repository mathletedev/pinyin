use std::{env::args, error::Error};

use clipboard_ext::{prelude::ClipboardProvider, x11_fork::ClipboardContext};
use reqwest::blocking::get;
use serde_json::{from_str, Value};

fn main() -> Result<(), Box<dyn Error>> {
	let args = args()
		.collect::<Vec<String>>()
		.splice(1.., [])
		.collect::<Vec<String>>();

	if args.len() == 0 {
		return Ok(());
	}

	let res = from_str::<Value>(get(format!("https://www.google.com/inputtools/request?ime=pinyin&ie=utf-8&oe=utf-8&app=translate&text={}", args.join(" ")))?.text()?.as_str())?;

	ClipboardContext::new()
		.unwrap()
		.set_contents(res[1][0][1][0].as_str().unwrap().to_string().to_owned())?;

	println!("{}", res[1][0][1][0].as_str().unwrap());

	Ok(())
}
