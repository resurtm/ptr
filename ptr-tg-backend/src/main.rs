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

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let get_me_resp = tg::get_me().await?;
    println!("{:#?}", get_me_resp);

    let mut max_update_id = None;
    loop {
        let get_updates_resp = tg::get_updates(increment_max_update_id(max_update_id)).await?;
        println!("{:#?}", get_updates_resp);
        max_update_id = get_updates_resp.get_max_update_id();
    }

    // Ok(())
}

fn increment_max_update_id(max_update_id: Option<i64>) -> Option<i64> {
    let x = max_update_id?;
    Some(x + 1)
}
