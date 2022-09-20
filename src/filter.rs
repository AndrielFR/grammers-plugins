// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2022 AndrielFR <https://github.com/AndrielFR>

use grammers_client::{types, Client, Update};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    Administrator,
    All,
    Chat(i64),
    Contact,
    Edited,
    Deleted,
    Document,
    Forward,
    Media,
    Mentioned,
    OutGoing,
    Photo,
    Raw,
    Reply,
    Regex(String),
    Text(String),
    Sticker,
}

impl Filter {
    pub(crate) async fn check(&self, client: &mut Client, update: &Update) -> bool {
        match self {
            Self::Administrator => match update {
                Update::CallbackQuery(callback) => {
                    let chat = callback.chat();

                    if let types::Chat::Group(group) = chat {
                        let user = callback.sender();

                        let permissions = client.get_permissions(group, user).await.unwrap();
                        permissions.is_admin()
                    } else {
                        false
                    }
                }
                Update::NewMessage(message) | Update::MessageEdited(message) => {
                    let chat = message.chat();

                    if let types::Chat::Group(group) = chat {
                        let user = message.sender().unwrap();

                        let permissions = client.get_permissions(group, user).await.unwrap();
                        permissions.is_admin()
                    } else {
                        false
                    }
                }
                _ => false,
            },
            Self::All => true,
            Self::Chat(chat_id) => {
                let current_id = match update {
                    Update::CallbackQuery(ref callback) => callback.chat().id(),
                    Update::NewMessage(ref message) | Update::MessageEdited(ref message) => message.chat().id(),
                    _ => 0,
                };

                current_id == *chat_id
            }
            Self::Contact => match update {
                Update::NewMessage(message) => {
                    if let Some(media) = message.media() {
                        if let types::Media::Contact(_) = media {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            },
            Self::Deleted => match update {
                Update::MessageDeleted(_) => true,
                _ => false,
            }
            Self::Document => match update {
                Update::NewMessage(message) | Update::MessageEdited(message) => {
                    if let Some(media) = message.media() {
                        if let types::Media::Document(_) = media {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            },
            Self::Edited => match update {
                Update::MessageEdited(_) => true,
                _ => false,
            }
            Self::Forward => match update {
                Update::NewMessage(ref message) => message.forward_header().is_some(),
                _ => false,
            },
            Self::Media => match update {
                Update::NewMessage(ref message) | Update::MessageEdited(ref message) => message.media().is_some(),
                _ => false,
            },
            Self::Mentioned => match update {
                Update::NewMessage(ref message) => message.mentioned(),
                _ => false,
            },
            Self::OutGoing => match update {
                Update::NewMessage(ref message) => message.outgoing(),
                _ => false,
            },
            Self::Photo => match update {
                Update::NewMessage(message) | Update::MessageEdited(message) => {
                    if let Some(media) = message.media() {
                        if let types::Media::Photo(_) = media {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            },
            Self::Raw => match update {
                Update::Raw(_) => true,
                _ => false,
            }
            Self::Reply => match update {
                Update::NewMessage(ref message) => message.reply_header().is_some(),
                _ => false,
            },
            Self::Regex(expr) => {
                let query = match update {
                    Update::CallbackQuery(ref callback) => {
                        std::str::from_utf8(callback.data()).unwrap()
                    }
                    Update::InlineQuery(ref inline) => inline.text(),
                    Update::NewMessage(ref message) | Update::MessageEdited(ref message) => message.text(),
                    _ => "",
                };

                let re = Regex::new(&expr).unwrap();
                re.is_match(query)
            }
            Self::Text(text) => match update {
                Update::NewMessage(message) | Update::MessageEdited(message) => message.text() == text,
                _ => false,
            },
            Self::Sticker => match update {
                Update::NewMessage(message) => {
                    if let Some(media) = message.media() {
                        if let types::Media::Sticker(_) = media {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            },
        }
    }
}
