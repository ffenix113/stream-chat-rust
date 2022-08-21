# Rust SDK for [Stream Chat](https://getstream.io/chat/)

[![Rust](https://github.com/ffenix113/stream-chat-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/ffenix113/stream-chat-rust/actions/workflows/rust.yml)

<p align="center">
    <img src="./assets/logo.svg" width="50%" height="50%">
</p>
<p align="center">
    Rust API client for Stream Chat, a service for building chat applications.
    <br />
    <a href="https://getstream.io/chat/docs/"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/ffenix113/stream-chat-rust/issues">Report Bug</a>
    ·
    <a href="https://github.com/ffenix113/stream-chat-rust/issues">Request Feature</a>
</p>

## 📝 About Stream

You can sign up for a Stream account at our [Get Started](https://getstream.io/chat/get_started/) page.

You can use this library to access chat API endpoints server-side.

For the client-side integrations (web and mobile) have a look at the JavaScript, iOS and Android SDK libraries ([docs](https://getstream.io/chat/)).

## ⚠️ Important

This is not concidered "official" SDK, as such does not have same support availability as official SDKs do.

For example only support currently possible for this SDK is though Github issues/discussions, 
and not through GetStream support.

If there is an issue with this SDK - please test with REST / official SDK to see if the issue is in this SDK.
If it is not - use official [https://getstream.io/contact/support/](support channel), if it is - use Github issues/discussions in this repo.

## State of this SDK

Rust SDK is in early development. No stability guarantees are provided.

Expect library to be broken, function signatures to change, functionality to be dropped/added, types changed etc.

## Installation

Add to your `Cargo.toml`:
```
[dependencies]
stream_chat_rust = { git = "https://github.com/ffenix113/stream-chat-rust", branch = "master" }
```

## Getting started

```rust
use stream_chat_rust::client::StreamChat;

fn main() {
    let api_key = String::from("api_key");
    let api_secret = String::from("api_secret");
    let chat = StreamChat::new(api_key, api_secret).unwrap();
    // Use `chat` as necessary.
}
```