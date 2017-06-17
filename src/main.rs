#![cfg(windows)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate clipboard_master;
extern crate clipboard_win;

use std::io;
use std::process::exit;

#[macro_use]
mod utils;

use clipboard_master::{
    Master,
    CallbackResult,
};

use clipboard_win::{
    Clipboard,
    formats
};

fn error_callback(error: io::Error) -> CallbackResult {
    error_println!("Error: {}", error);
    CallbackResult::Next
}

fn remove_color(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE_TAG: regex::Regex = regex::Regex::new("<[^>]+>").unwrap();
    }

    let result = RE_TAG.replace_all(text, "");

    if result.len() != text.len() {
        Some(result.to_string())
    }
    else {
        None
    }
}

///Processes text and returns changed text or None.
fn process_text(text: &str) -> Option<String>{
    if utils::is_jp(text) {
        remove_color(text).map(|text| utils::remove_text_reps(text).replace("\n", ""))
    }
    else {
        None
    }
}

fn ok_callback() -> CallbackResult {
    const RES: CallbackResult = CallbackResult::Next;

    if !Clipboard::is_format_avail(formats::CF_UNICODETEXT) {
        return RES;
    }

    match Clipboard::new() {
        Ok(clip) => {
            match clip.get_string() {
                Ok(content) => {
                    if let Some(new_text) = process_text(&content) {
                        if let Err(error) = clip.set_string(&new_text) {
                            error_println!("Failed to set clipboard content. Error: {}", error);
                        }
                    }
                }
                Err(error) => {
                    error_println!("Failed to get clipboard content. Error: {}", error);
                }
            }
        }
        Err(error) => {
            error_println!("Failed to open clipboard. Error: {}", error);
        }
    }

    RES
}

fn main() {
    match Master::new(ok_callback, error_callback).run() {
        Ok(_) => (),
        Err(error) => {
            error_println!("Aborted. Error: {}", error);
            exit(1)
        }
    }
}
