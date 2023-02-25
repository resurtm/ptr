// https://opensource.org/license/mit/
//
// Copyright 2023 resurtm@gmail.com
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
// documentation files (the “Software”), to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
// CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use serde::{Deserialize, Serialize};

/// Implementation for the GetUpdatesResponse DTO.
impl GetUpdatesResponse {
    fn get_max_update_id(&self) -> Option<i64> {
        match &self.result {
            Some(items) => {
                let item = items.iter().max_by_key(|p| p.update_id)?;
                Some(item.update_id)
            }
            None => None,
        }
    }

    pub fn get_next_update_id(&self) -> Option<i64> {
        let x = self.get_max_update_id()?;
        Some(x + 1)
    }
}

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#update>
/// Root main document.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUpdatesResponse {
    ok: bool,
    result: Option<Vec<GetUpdatesResponseResult>>,
}

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#update>
/// Result sub-key.
#[derive(Debug, Serialize, Deserialize)]
struct GetUpdatesResponseResult {
    update_id: i64,
}

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#user>
/// Root main document.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetMeResponse {
    ok: bool,
    error_code: Option<i16>,
    description: Option<String>,
    result: Option<GetMeResponseResult>,
}

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#user>
/// Result sub-key.
#[derive(Debug, Serialize, Deserialize)]
struct GetMeResponseResult {
    id: i64,
    is_bot: bool,
    first_name: String,
    username: String,
    can_join_groups: bool,
    can_read_all_group_messages: bool,
    supports_inline_queries: bool,
}
