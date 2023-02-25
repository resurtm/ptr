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

use std::error::Error;

pub struct Client {
    access_token: String,
}

impl Client {
    pub fn new(access_token: String) -> Client {
        Client { access_token }
    }

    pub async fn get_updates(
        &self,
        offset: Option<i64>,
    ) -> Result<dto::GetUpdatesResponse, Box<dyn Error>> {
        let mut parts = vec!["timeout=120".to_string()];
        if let Some(x) = offset {
            parts.push(format!("offset={}", x));
        }
        let get_updates_url = format!(
            "https://api.telegram.org/bot{}/getUpdates?{}",
            self.access_token,
            parts.join("&")
        );
        let resp: dto::GetUpdatesResponse = reqwest::get(get_updates_url).await?.json().await?;
        Ok(resp)
    }

    pub async fn get_me(&self) -> Result<dto::GetMeResponse, Box<dyn Error>> {
        let get_me_url = format!("https://api.telegram.org/bot{}/getMe", self.access_token);
        let resp: dto::GetMeResponse = reqwest::get(get_me_url).await?.json().await?;
        Ok(resp)
    }
}
