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

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#update>.
pub mod get_updates {
    use serde::{Deserialize, Serialize};

    /// Implementation for the GetUpdatesResponse DTO.
    impl Response {
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

    /// Root main document.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        pub ok: bool,
        pub result: Option<Vec<Result>>,
    }

    /// Result sub-key.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Result {
        pub update_id: i64,
        pub message: Message,
    }

    /// Result sub-key, message sub-key.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Message {
        pub message_id: i64,
        pub from: From,
        pub chat: Chat,
        pub date: i64,
        pub text: String,
    }

    /// Result sub-key, message sub-key, from sub-key.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct From {
        pub id: i64,
        pub is_bot: bool,
        pub first_name: String,
        pub last_name: String,
        pub username: String,
    }

    /// Result sub-key, message sub-key, chat sub-key.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Chat {
        pub id: i64,
        /// Examples: "private", "group".
        #[serde(rename = "type")]
        pub type_: String,
        /// Applies for the "group" type.
        pub title: Option<String>,
        /// Applies for the "group" type.
        pub all_members_are_administrators: Option<bool>,
        /// Applies for the "private" type.
        pub first_name: Option<String>,
        /// Applies for the "private" type.
        pub last_name: Option<String>,
        /// Applies for the "private" type.
        pub username: Option<String>,
    }
}

/// Represents the following Telegram Bot API data type:
/// <https://core.telegram.org/bots/api#user>.
pub mod get_me {
    use serde::{Deserialize, Serialize};

    /// Root main document.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Response {
        ok: bool,
        error_code: Option<i16>,
        description: Option<String>,
        result: Option<Result>,
    }

    /// Result sub-key.
    #[derive(Debug, Serialize, Deserialize)]
    struct Result {
        id: i64,
        is_bot: bool,
        first_name: String,
        username: String,
        can_join_groups: bool,
        can_read_all_group_messages: bool,
        supports_inline_queries: bool,
    }
}

/// Module contains unit tests for the DTO related code.
#[cfg(test)]
mod tests {
    use super::mocks::{get_updates_response_variation1, get_updates_response_variation2};

    /// Tests the `GetUpdatesResponse::get_next_update_id` function. Case 1.
    #[test]
    fn get_next_update_id_case1() {
        let data = get_updates_response_variation1();
        assert_eq!(data.get_next_update_id(), None);
    }

    /// Tests the `GetUpdatesResponse::get_next_update_id` function. Case 2.
    #[test]
    fn get_next_update_id_case2() {
        let data = get_updates_response_variation2();
        assert_eq!(data.get_next_update_id(), Some(151));
    }
}

/// Module contains mocks for different kind of tasks (tests, etc.).
#[allow(dead_code)]
mod mocks {
    use super::get_updates as upd;

    /// Generates a mocked instance of the `GetUpdatesResponse` object. Variation 1.
    pub fn get_updates_response_variation1() -> upd::Response {
        upd::Response {
            ok: true,
            result: None,
        }
    }

    /// Generates a mocked instance of the `GetUpdatesResponse` object. Variation 2.
    pub fn get_updates_response_variation2() -> upd::Response {
        let from = upd::From {
            id: 3001,
            is_bot: false,
            first_name: "John".to_string(),
            last_name: "Preston".to_string(),
            username: "john_preston".to_string(),
        };
        let chat1 = upd::Chat {
            id: 3002,
            type_: "group".to_string(),
            title: Some("FloodChat123".to_string()),
            all_members_are_administrators: Some(true),
            first_name: None,
            last_name: None,
            username: None,
        };
        let chat2 = upd::Chat {
            id: 3003,
            type_: "private".to_string(),
            title: None,
            all_members_are_administrators: None,
            first_name: Some("John".to_string()),
            last_name: Some("Preston".to_string()),
            username: Some("john_preston".to_string()),
        };
        let result = vec![
            upd::Result {
                update_id: 100,
                message: upd::Message {
                    message_id: 301,
                    from: from.clone(),
                    chat: chat1,
                    date: 1500,
                    text: "message one".to_string(),
                },
            },
            upd::Result {
                update_id: 150,
                message: upd::Message {
                    message_id: 302,
                    from,
                    chat: chat2,
                    date: 1515,
                    text: "Second Message".to_string(),
                },
            },
        ];
        upd::Response {
            ok: true,
            result: Some(result),
        }
    }
}
