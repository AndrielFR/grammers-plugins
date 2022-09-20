// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2022 AndrielFR <https://github.com/AndrielFR>

use grammers_client::types::{CallbackQuery, InlineQuery, Message};
use grammers_tl_types::enums::Update;

use super::HandlerType;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct Data {
    pub callback_query: Option<CallbackQuery>,
    pub inline_query: Option<InlineQuery>,
    pub message: Option<Message>,
    pub raw: Option<Update>,
    pub update_type: HandlerType,
    pub user_id: Option<i64>,
    pub chat_id: Option<i64>,
    pub query: Option<String>,
}
