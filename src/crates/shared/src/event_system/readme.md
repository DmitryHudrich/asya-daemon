# Guide to Publishing and Subscribing to Events in Asya internal logic

The Asya helper utilizes an event system to facilitate communication through events. 

## Publishing Events

To publish an event, use the `event_system::publish` method.
Here's an example of publishing an event of type `AsyaResponse::Ok` with a custom message:

```rust
use event_system::publish;

publish(AsyaResponse::Ok {
    message: res.to_string(),
});
```

In this example:
- `AsyaResponse` is an enumeration that defines the type of events.
- The event data, such as the `message`, is passed to the `publish` function.


### Asya already has several predefined events. Here are descriptions of some of them:

- General response event. Use it for sending messages to the client:

```rust
AsyaResponse::Ok { message: String }
```

- Hardware status event. Regularly published to notify about the system's status.

```rust
pub enum HardwareStatus {
    Ok { cpu_usage: f32, mem_usage: u64 },
}
```

You can define other variants of the `AsyaResponse` and `HardwareStatus` enums and publish events accordingly.

## Subscribing to Events

To subscribe to events, use the `event_system::subscribe_once` function.
Here's an example of how a WebSocket server listens for an AsyaResponse event and sends it to the client:

```rust
use event_system::subscribe_once;
use std::sync::Arc;
use async_std::task;

subscribe_once(
    {
        let session = session.clone();
        move |event: Arc<AsyaResponse>| {
            let mut session = session.clone();
            task::spawn(async move {
                let response = Responses::Base {
                    is_err: false,
                    message: event.to_string(),
                };

                session
                    .text(
                        serde_json::to_string(&response)
                            .expect(DEFAULT_EXPECT_MSG)
                            .to_string(),
                    )
                    .await
                    .unwrap();
            })
        }
    }
)
.await;
```

## Event Data Structure

An event can be any variant of an enumeration that implement `Debug` and `Display`, such as `AsyaResponse`. For example:

```rust
#[derive(Debug, parse_display::Display)]
enum AsyaResponse {
    #[display("{message}")]
    Ok { message: String },
    // Add more variants as needed.
}
```

You can define custom variants and include relevant data for your application.

Good luck, cowboy


```
⠀⠀⠀⣸⠁⢰⠃⠀⠀⠀⠈⢣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣇
⠀⠀⠀⡇⠀⡾⡀⠀⠀⠀⠀⣀⣹⣆⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹
⠀⠀⢸⠃⢀⣇⡈⠀⠀⠀⠀⠀⠀⢀⡑⢄⡀⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇
⠀⠀⢸⠀⢻⡟⡻⢶⡆⠀⠀⠀⠀⡼⠟⡳⢿⣦⡑⢄⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇
⠀⠀⣸⠀⢸⠃⡇⢀⠇⠀⠀⠀⠀⠀⡼⠀⠀⠈⣿⡗⠂⠀⠀⠀⠀⠀⠀⠀⢸⠁
⠀⠀⡏⠀⣼⠀⢳⠊⠀⠀⠀⠀⠀⠀⠱⣀⣀⠔⣸⠁⠀⠀⠀⠀⠀⠀⠀⢠⡟
⠀⠀⡇⢀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⢸⠃
⠀⢸⠃⠘⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠁⠀⠀⢀⠀⠀⠀⠀⠀⣾
⠀⣸⠀⠀⠹⡄⠀⠀⠈⠁⠀⠀⠀⠀⠀⠀⠀⡞⠀⠀⠀⠸⠀⠀⠀⠀⠀⡇
⠀⡏⠀⠀⠀⠙⣆⠀⠀⠀⠀⠀⠀⠀⢀⣠⢶⡇⠀⠀⢰⡀⠀⠀⠀⠀⠀|
⢰⠇⡄⠀⠀⠀⡿⢣⣀⣀⣀⡤⠴⡞⠉⠀⢸⠀⠀⠀⣿⡇⠀⠀⣧⣸ 
```
