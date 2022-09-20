// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2022 AndrielFR <https://github.com/AndrielFR>

mod data;
mod filter;
pub mod filters;
mod handler;
pub mod handlers;
mod plugin;

pub use data::Data;
pub(crate) use filter::Filter;
use grammers_client::{Client, Update};
pub use handler::{Handler, HandlerType, OtherHandler};
pub use plugin::Plugin;

pub(crate) type Result = std::result::Result<(), Box<dyn std::error::Error>>;

use once_cell::sync::OnceCell;

static USERNAME: OnceCell<Option<String>> = OnceCell::new();
static PREFIXES: OnceCell<Vec<String>> = OnceCell::new();

pub async fn run(mut client: Client, plugins: Vec<Plugin>, prefixes: Vec<String>) -> Result {
    let me = client.get_me().await.unwrap();
    let username = me.username();
    let _ = USERNAME.set(username.map(String::from));
    let _ = PREFIXES.set(prefixes.clone());

    while let Some(update) = tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(None),
        result = client.next_update() => result,
    }? {
        let mut client = client.clone();
        let plugins = plugins.clone();

        tokio::task::spawn(async move {
            for plugin in plugins.iter() {
                if let Some(handler) = plugin.get_handler(&mut client, &update).await {
                    if handler.check(&mut client, &update).await {
                        let mut data = Data::default();

                        match update {
                            Update::CallbackQuery(callback_query) => {
                                data.chat_id = callback_query.chat().id();
                                data.user_id = callback_query.sender().id();
                                data.query = std::str::from_utf8(callback_query.data())
                                    .unwrap()
                                    .to_string();
                                data.update_type = HandlerType::CallbackQuery;
                                data.callback_query = Some(callback_query);
                            }
                            Update::InlineQuery(inline_query) => {
                                data.user_id = inline_query.sender().id();
                                data.chat_id = data.user_id;
                                data.query = inline_query.text().to_string();
                                data.update_type = HandlerType::InlineQuery;
                                data.inline_query = Some(inline_query);
                            }
                            Update::NewMessage(message) | Update::MessageEdited(message) => {
                                data.chat_id = message.chat().id();
                                data.user_id = message.sender().unwrap().id();
                                data.query = message.text().to_string();
                                data.update_type = HandlerType::Message;
                                data.message = Some(message);
                            }
                            Update::Raw(raw) => {
                                data.update_type = HandlerType::Raw;
                                data.raw = Some(raw);
                            }
                            _ => {}
                        }

                        handler
                            .run(client, data)
                            .await
                            .expect("handler failed to run");
                        break;
                    }
                }
            }
        });
    }

    Ok(())
}

pub fn username() -> String {
    USERNAME.get().unwrap().clone().unwrap_or("".to_owned())
}

pub fn prefixes() -> &'static [String] {
    PREFIXES.get().unwrap().as_slice()
}
