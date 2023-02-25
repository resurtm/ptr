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

mod tg;

use config::Config;
use std::error::Error;

/// Main application entry point function.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new()?;
    let result = app.run().await?;
    Ok(result)
}

struct App {
    tg: tg::Client,
}

impl App {
    fn new() -> Result<App, Box<dyn Error>> {
        let config = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .add_source(config::Environment::with_prefix("PTR_TG_BACKEND"))
            .build()?;
        let access_token = config.get_string("access_token")?;
        let tg = tg::Client::new(access_token);
        Ok(App { tg })
    }

    async fn run(&self) -> Result<(), Box<dyn Error>> {
        self.run_get_me().await?;
        self.run_main_loop().await?;
        Ok(())
    }

    async fn run_main_loop(&self) -> Result<(), Box<dyn Error>> {
        let mut next_update_id = None;
        loop {
            let get_updates_resp = self
                .tg
                .get_updates(next_update_id)
                .await?;
            println!("Telegram getUpdates response: {:#?}", get_updates_resp);
            next_update_id = get_updates_resp.get_next_update_id();
        }
    }

    async fn run_get_me(&self) -> Result<(), Box<dyn Error>> {
        let get_me_resp = self.tg.get_me().await?;
        println!("Telegram getMe response: {:#?}", get_me_resp);
        Ok(())
    }
}
