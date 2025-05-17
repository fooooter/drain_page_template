use drain_common::RequestData::*;
use drain_common::sessions::{SessionValue};
use drain_macros::{drain_endpoint, set_header, start_session, SessionValue};

#[derive(Clone, SessionValue)]
pub struct Counter(u32);

#[drain_endpoint("index")]
pub fn index() {
    let mut session = start_session!().await;

    let Counter(mut counter) = session.get::<Counter>(&String::from("counter")).await.unwrap_or(Counter(0));

    let content: Vec<u8> = Vec::from(format!(r#"
    <!DOCTYPE html>
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Index</title>
        </head>
        <body>
            Hello, world! {} request was sent.<br>
            Counter: {}
        </body>
    </html>"#, match REQUEST_DATA {
        Get(_) => "GET",
        Post{..} => "POST",
        Head(_) => "HEAD",
        Put{..} => "PUT",
        Delete{..} => "DELETE",
        Patch{..} => "PATCH"
    }, counter));

    counter += 1;
    session.set(String::from("counter"), Box::new(Counter(counter))).await;

    set_header!("Content-Type", "text/html; charset=utf-8");

    Some(content)
}