// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2022 AndrielFR <https://github.com/AndrielFR>

pub use super::Filter::All as all;
pub use super::Filter::Contact as contact;
pub use super::Filter::Deleted as deleted;
pub use super::Filter::Document as document;
pub use super::Filter::Edited as edited;
pub use super::Filter::Forward as forward;
pub use super::Filter::Media as media;
pub use super::Filter::Mentioned as mentioned;
pub use super::Filter::OutGoing as outgoing;
pub use super::Filter::Photo as photo;
pub use super::Filter::Raw as raw;
pub use super::Filter::Reply as reply;
pub use super::Filter::Sticker as sticker;

pub fn command(pattern: &'static str) -> super::Filter {
    let mut expr = pattern.to_string();
    let mut has_final_line_symbol = false;

    if expr.ends_with('$') {
        expr.pop();
        has_final_line_symbol = true;
    }

    let expr_clone = expr.clone();
    let expr_splitted = expr_clone.split_ascii_whitespace().collect::<Vec<&str>>();

    if expr_splitted.len() > 1 {
        expr.clear();
        expr.push_str(&expr_splitted[..1].join(" "));
    }

    expr.push_str(format!("(?:@{})?", super::username()).as_str());
    expr.insert_str(0, format!("^[{}]", super::prefixes().join("")).as_str());

    if has_final_line_symbol {
        expr.push('$');
    }

    super::Filter::Regex(expr)
}

pub fn chat(chat_id: i64) -> super::Filter {
    super::Filter::Chat(chat_id)
}

pub fn regex(expr: &'static str) -> super::Filter {
    super::Filter::Regex(expr.to_string())
}

pub fn text(equal: &'static str) -> super::Filter {
    super::Filter::Text(equal.to_string())
}
