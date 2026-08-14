# tgrs

A small, typed asynchronous Rust client for the Telegram Bot API. It provides a
focused set of request builders and response types without imposing a bot
framework or runtime.

```rust,no_run
use tgrs::*;

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let telegram = Telegram::new("BOT_TOKEN")?;
let request = SendMessage::builder().chat_id(123456789).text("Hello!").build();
let _: Message = request.send(&telegram).await?;
# Ok(())
# }
```

## Licensing

Dual-licensed under the **MIT** and **Apache License 2.0**. See the included
`LICENSE` file.
