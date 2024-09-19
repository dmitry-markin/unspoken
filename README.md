**NOTICE** This project has been renamed to `jutella` and has been moved to https://github.com/dmitry-markin/jutella. This crate/binary has been moved to [jutella](https://crates.io/crates/jutella) from `0.2.0`


# unspoken

Chatbot API client library and CLI interface. Currently supports OpenAI chat API.


## Command line interface

To get started with CLI, put your API key and endpoint into `~/.config/unspoken.toml`. See a config [example](https://github.com/dmitry-markin/unspoken/blob/master/config/unspoken.toml).

![Screenshot](doc/screenshot.png)


## Library

To use the chat API, initialize `ChatClient` with `api_key` and `ChatClientConfig`:

```rust
let mut chat = ChatClient::new(api_key, ChatClientConfig::default());
```

Request answers via `ChatClient::ask()`:

```rust
let answer = chat.ask("What is the highest point on Earth?".to_string())?;
println!("The answer is: {answer}");
```

`ChatClient` keeps the conversation context and sends it with every `ask()` to the chatbot API.


## Future plans

Expect breaking changes in the API and transition to async requests.
