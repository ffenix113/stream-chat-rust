#[macro_use]
extern crate serde_derive;

pub mod client;
pub mod endpoints;
pub mod models;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::models::ApiError;

    use super::*;

    #[test]
    fn construct_client() {
        let key = String::from("key");
        let secret = String::from("secret");
        let client = client::StreamChat::new(key, secret);

        match client {
            Some(_) => assert!(true),
            None => assert!(false),
        }
    }

    #[tokio::test]
    async fn send_get_request() {
        let key = String::from("key");
        let secret = String::from("secret");

        std::env::set_var("STREAM_CHAT_URL", "https://httpbin.org");
        let client = client::StreamChat::new(key, secret);

        match client {
            Some(_) => assert!(true),
            None => assert!(false),
        }

        let client = client.unwrap();

        type Resp = HashMap<String, serde_json::Value>;

        let resp = client
            .get::<_, Resp>("anything", Some(&[("key1", "val1"), ("key2", "val2")]))
            .await
            .unwrap();

        for (key, val) in &resp {
            println!("{}: {}", key, val)
        }
    }

    #[tokio::test]
    async fn send_post_request() {
        let key = String::from("key");
        let secret = String::from("secret");

        std::env::set_var("STREAM_CHAT_URL", "https://httpbin.org");
        let client = client::StreamChat::new(key, secret);

        match client {
            Some(_) => assert!(true),
            None => assert!(false),
        }

        let client = client.unwrap();

        type Resp = HashMap<String, serde_json::Value>;

        let resp = client
            .post::<ApiError, Resp>(
                "anything",
                &ApiError {
                    code: Some(1),
                    status_code: None,
                    details: None,
                    duration: None,
                    exception_fields: None,
                    message: None,
                    more_info: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(resp.get("json").unwrap().get("code").unwrap(), 1);
    }
}
