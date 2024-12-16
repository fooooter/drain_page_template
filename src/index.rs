use std::collections::HashMap;
use drain_common::RequestData::{self, *};

#[export_name = "index"]
pub fn index(request_data: RequestData, response_headers: &mut HashMap<String, String>) -> Option<Vec<u8>> {
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

    response_headers.insert(String::from("Content-Type"), String::from("text/html; charset=utf-8"));

    Some(content)
}