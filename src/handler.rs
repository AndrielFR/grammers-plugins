// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2022 AndrielFR <https://github.com/AndrielFR>

use std::{future::Future, pin::Pin};

use grammers_client::{Client, Update};

use async_recursion::async_recursion;

use super::{Data, Filter, Result};

pub type AsyncFunction = fn(Client, Data) -> Pin<Box<dyn Future<Output = Result> + Send + 'static>>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HandlerType {
    CallbackQuery,
    InlineQuery,
    Message,
    MessageDeleted,
    Raw,
}

impl Default for HandlerType {
    fn default() -> Self {
        Self::Raw
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Handler {
    types: Vec<HandlerType>,
    filters: Vec<Filter>,
    function: AsyncFunction,
    other: Option<Box<OtherHandler>>,
}

impl Handler {
    #[async_recursion]
    pub(crate) async fn check(&self, client: &mut Client, update: &Update) -> bool {
        let mut result = false;

        match update {
            Update::CallbackQuery(_) => {
                if !self.types.contains(&HandlerType::CallbackQuery) {
                    result = false;
                }
            }
            Update::InlineQuery(_) => {
                if !self.types.contains(&HandlerType::InlineQuery) {
                    result = false;
                }
            }
            Update::NewMessage(_) | Update::MessageEdited(_) => {
                if !self.types.contains(&HandlerType::Message) {
                    result = false;
                }
            }
            Update::MessageDeleted(_) => {
                if !self.types.contains(&HandlerType::MessageDeleted) {
                    result = false;
                }
            }
            Update::Raw(_) => {
                if !self.types.contains(&HandlerType::Raw) {
                    result = false;
                }
            }
            _ => {
                result = false;
            }
        }

        if !result {
            if let Some(other) = &self.other {
                match **other {
                    OtherHandler::Or(ref handler) => return handler.check(client, update).await,
                    _ => {}
                }
            }
        }

        for filter in self.filters.iter() {
            if filter.check(client, update).await {
                result = true;

                if let Some(other) = &self.other {
                    match **other {
                        OtherHandler::And(ref handler) => match handler.check(client, update).await
                        {
                            true => return true,
                            false => {
                                result = false;
                            }
                        },
                        _ => {}
                    }
                }
            }
        }

        result
    }

    pub async fn run(&self, client: Client, data: Data) -> Result {
        (self.function)(client, data).await
    }

    pub fn filters(&self) -> &[Filter] {
        &self.filters
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OtherHandler {
    And(Handler),
    Or(Handler),
}
