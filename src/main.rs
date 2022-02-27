use std::{env::args, error::Error};

use clipboard_ext::{prelude::ClipboardProvider, x11_fork::ClipboardContext};
use druid::{
	im::Vector,
	widget::{Align, Button, Flex, Label, List, TextBox},
	AppLauncher, Data, Env, EventCtx, FontDescriptor, FontFamily, Lens, Widget, WidgetExt,
	WindowDesc,
};
use reqwest::blocking::get;
use serde_json::{from_str, Value};

const FONT_FAMILY: &str = "WenQuanYi Zen Hei";
const FONT_SIZE: f64 = 24.0;

const TEXT_BOX_WIDTH: f64 = 256.0;

#[derive(Clone, Data, Lens)]
struct AppState {
	pinyin: String,
	hanzi: Vector<HanziItem>,
}

#[derive(Clone, Data, Lens)]
struct HanziItem {
	text: String,
}

fn req(text: String, many: bool) -> Result<Value, Box<dyn Error>> {
	Ok(from_str::<Value>(get(format!("https://www.google.com/inputtools/request?ime=pinyin&ie=utf-8&oe=utf-8&app=translate&{}text={}", if many { "num=10&" } else { "" }, text))?.text()?.as_str())?)
}

fn hanzi_item() -> impl Widget<HanziItem> {
	Label::new(|data: &HanziItem, _env: &Env| data.text.to_owned())
		.with_font(FontDescriptor::new(FontFamily::new_unchecked(FONT_FAMILY)))
		.with_text_size(FONT_SIZE)
		.on_click(|_ctx: &mut EventCtx, data: &mut HanziItem, _env: &Env| {
			ClipboardContext::new()
				.unwrap()
				.set_contents(data.text.to_owned())
				.unwrap();
		})
}

fn build() -> impl Widget<AppState> {
	Align::centered(
		Flex::column()
			.with_child(
				TextBox::new()
					.with_text_size(FONT_SIZE)
					.with_placeholder("Pinyin")
					.fix_width(TEXT_BOX_WIDTH)
					.lens(AppState::pinyin),
			)
			.with_child(
				Button::new("Translate!")
					.on_click(|_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
						data.hanzi = req(data.pinyin.to_owned(), true).unwrap()[1][0][1]
							.as_array()
							.or(Some(&vec![]))
							.unwrap()
							.to_owned()
							.iter()
							.map(|h| HanziItem {
								text: h.as_str().unwrap().to_string(),
							})
							.collect::<Vector<HanziItem>>();
					})
					.fix_width(TEXT_BOX_WIDTH),
			)
			.with_child(
				List::new(hanzi_item)
					.lens(AppState::hanzi)
					.fix_width(TEXT_BOX_WIDTH),
			),
	)
}

fn cl(text: String) -> Result<(), Box<dyn Error>> {
	let res = req(text, false)?;

	ClipboardContext::new()
		.unwrap()
		.set_contents(res[1][0][1][0].as_str().unwrap().to_string().to_owned())?;

	println!("{}", res[1][0][1][0].as_str().unwrap());

	Ok(())
}

fn gui() -> Result<(), Box<dyn Error>> {
	AppLauncher::with_window(WindowDesc::new(build).title("Pinyin").resizable(false)).launch(
		AppState {
			pinyin: "".into(),
			hanzi: Vector::new(),
		},
	)?;

	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	let args = args()
		.collect::<Vec<String>>()
		.splice(1.., [])
		.collect::<Vec<String>>();

	if args.len() == 0 {
		return gui();
	}

	cl(args.join(" "))
}
