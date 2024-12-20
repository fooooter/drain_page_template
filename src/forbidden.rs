use std::collections::HashMap;
use drain_common::RequestData;
use drain_common::cookies::SetCookie;
use drain_macros::*;

#[export_name = "forbidden"]
#[drain_page]
pub fn forbidden() -> Option<Vec<u8>> {
    let content: Vec<u8> = Vec::from(r#"
    <!DOCTYPE html>
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>403</title>
        </head>
        <body>
            <h2>403 Forbidden</h2>
        </body>
    </html>"#
    );

    set_header!("Content-Type", "text/html; charset=utf-8");

    Some(content)
}