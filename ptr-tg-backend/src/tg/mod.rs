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

mod dto;

use dto::{get_me::Response as GetMe, get_updates::Response as Upd};
use std::error::Error;

/// The Telegram Bot API client implementation details.
impl Client {
    /// Create a new instance of the Telegram Bot API client.
    ///
    /// # Arguments
    ///
    /// * `access_token` - A string which contains the Telegram Bot API access token value.
    /// * `long_polling_timeout` - An integer value which contains long polling timeout value.
    pub fn new(access_token: String, long_polling_timeout: u16) -> Client {
        Client {
            access_token,
            long_polling_timeout,
        }
    }

    /// Represents the getUpdates Telegram Bot API method.
    /// See here for more details: <https://core.telegram.org/bots/api#getupdates>.
    ///
    /// # Arguments
    ///
    /// * `offset` - Identifier of the first update to be returned.
    pub async fn get_updates(&self, offset: Option<i64>) -> Result<Upd, Box<dyn Error>> {
        let mut get_params = vec![format!("timeout={}", self.long_polling_timeout)];
        if let Some(offset_val) = offset {
            get_params.push(format!("offset={}", offset_val));
        }
        let get_updates_url = format!(
            "{}{}/getUpdates?{}",
            Self::API_BASE_URL,
            self.access_token,
            get_params.join("&")
        );
        let resp: Upd = reqwest::get(get_updates_url).await?.json().await?;
        Ok(resp)
    }

    /// Represents the getMe Telegram Bot API method.
    /// See here for more details: <https://core.telegram.org/bots/api#getme>.
    pub async fn get_me(&self) -> Result<GetMe, Box<dyn Error>> {
        let get_me_url = format!("{}{}/getMe", Self::API_BASE_URL, self.access_token);
        let resp: GetMe = reqwest::get(get_me_url).await?.json().await?;
        Ok(resp)
    }

    /// Telegram Bot API base URL.
    const API_BASE_URL: &str = "https://api.telegram.org/bot";
}

/// Represents the Telegram Bot API client with a bunch of API calls/methods.
pub struct Client {
    /// Telegram Bot API access token.
    /// See more details here: <https://t.me/BotFather>.
    access_token: String,
    /// Long polling timeout value, for getUpdates Telegram Bot API call/method.
    /// See here for more details: <https://core.telegram.org/bots/api#getupdates>.
    long_polling_timeout: u16,
}
