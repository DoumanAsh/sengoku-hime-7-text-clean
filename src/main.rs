#![cfg(windows)]

extern crate clipboard_master;
extern crate clipboard_win;

#[macro_use]
extern crate utils;

use std::io;
use std::process::exit;


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

fn open_clipboard() -> Clipboard {
    loop {
        match Clipboard::new() {
            Ok(clipboard) => return clipboard,
            Err(error) => error_println!("Failed to open clipboard. Error: {}", error)
        }
    }
}

fn get_clipboard_string(clip: &Clipboard) -> String {
    loop {
        match clip.get_string() {
            Ok(content) => return content,
            Err(error) => error_println!("Failed to get content from Clipboard. Error: {}", error)
        }
    }
}

fn set_clipboard_string(clip: &Clipboard, content: &str) {
    loop {
        match clip.set_string(content) {
            Ok(_) => break,
            Err(error) => error_println!("Failed to set content onto Clipboard. Error: {}", error)
        }
    }
}

fn ok_callback() -> CallbackResult {
    const RES: CallbackResult = CallbackResult::Next;

    if !Clipboard::is_format_avail(formats::CF_UNICODETEXT) {
        return RES;
    }

    let clip = open_clipboard();
    let content = get_clipboard_string(&clip);

    if let Some(new_text) = utils::process_text(content) {
        set_clipboard_string(&clip, &new_text)
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
