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
    /// Finds the maximal Update ID value in the GetUpdatesResponse results.
    fn get_max_update_id(&self) -> Option<i64> {
        match &self.result {
            Some(results) => {
                let result = results.iter().max_by_key(|p| p.update_id)?;
                Some(result.update_id)
            }
            None => None,
        }
    }

    /// Finds the next Update ID value to be used to fetch the Telegram Bot API updates.
    /// Related Telegram Bot API call with some more information:
    /// <https://core.telegram.org/bots/api#getupdates>.
    pub fn get_next_update_id(&self) -> Option<i64> {
        let max_update_id = self.get_max_update_id()?;
        Some(max_update_id + 1)
    }
}

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#update>.
/// Root main document.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetUpdatesResponse {
    ok: bool,
    result: Option<Vec<GetUpdatesResponseResult>>,
}

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#update>.
/// Result sub-key.
#[derive(Debug, Serialize, Deserialize)]
struct GetUpdatesResponseResult {
    update_id: i64,
}

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#user>.
/// Root main document.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetMeResponse {
    ok: bool,
    error_code: Option<i16>,
    description: Option<String>,
    result: Option<GetMeResponseResult>,
}

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#user>.
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

/// Module contains unit tests for the DTO related code.
#[cfg(test)]
mod tests {
    /// Tests the `GetUpdatesResponse::get_next_update_id` function. Case 1.
    #[test]
    fn get_next_update_id_case1() {
        let data = super::mocks::get_updates_response_variation1();
        assert_eq!(data.get_next_update_id(), None);
    }

    /// Tests the `GetUpdatesResponse::get_next_update_id` function. Case 2.
    #[test]
    fn get_next_update_id_case2() {
        let data = super::mocks::get_updates_response_variation2();
        assert_eq!(data.get_next_update_id(), Some(151));
    }
}

/// Module contains mocks for different kind of tasks (tests, etc.).
#[allow(dead_code)]
mod mocks {
    /// Generates a mocked instance of the `GetUpdatesResponse` object. Variation 1.
    pub fn get_updates_response_variation1() -> super::GetUpdatesResponse {
        super::GetUpdatesResponse {
            ok: true,
            result: None,
        }
    }

    /// Generates a mocked instance of the `GetUpdatesResponse` object. Variation 2.
    pub fn get_updates_response_variation2() -> super::GetUpdatesResponse {
        super::GetUpdatesResponse {
            ok: true,
            result: Some(vec![
                super::GetUpdatesResponseResult { update_id: 100 },
                super::GetUpdatesResponseResult { update_id: 150 },
            ]),
        }
    }
}
