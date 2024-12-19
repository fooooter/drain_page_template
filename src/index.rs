use std::collections::HashMap;
use drain_common::RequestData::{self, *};
use drain_common::cookies::SetCookie;
use drain_macros::*;

#[export_name = "index"]
#[drain_page]
pub fn index() -> Option<Vec<u8>> {
    let content: Vec<u8> = Vec::from(format!(r#"
    <!DOCTYPE html>
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Index</title>
        </head>
        <body>
            Hello, world! {} request was sent.
        </body>
    </html>"#, match request_data {
        Get {..} => "GET",
        Post {..} => "POST",
        Head {..} => "HEAD"
    }));

    header!("Content-Type", "text/html; charset=utf-8");

    Some(content)
}