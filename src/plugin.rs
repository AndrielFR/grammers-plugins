// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2022 AndrielFR <https://github.com/AndrielFR>

use super::Handler;

use grammers_client::{Client, Update};

#[derive(Debug, Clone, PartialEq)]
pub struct Plugin {
    name: String,
    handlers: Vec<Handler>,
    file_path: String,
}

impl Plugin {
    pub fn new(name: String, file_path: String) -> Self {
        Self {
            name,
            handlers: Vec::new(),
            file_path,
        }
    }

    pub fn push_handler(&mut self, handler: Handler) {
        self.handlers.push(handler)
    }

    pub(crate) async fn get_handler(
        &self,
        client: &mut Client,
        update: &Update,
    ) -> Option<&Handler> {
        for handler in self.handlers.iter() {
            if handler.check(client, update).await {
                return Some(handler);
            }
        }

        None
    }

    pub fn handlers(&self) -> &[Handler] {
        &self.handlers
    }

    pub fn file_path(&self) -> &str {
        self.file_path.as_str()
    }
}
