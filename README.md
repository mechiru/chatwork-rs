# chatwork

A client implementation of [Chatwork](https://go.chatwork.com/) API.


## Example

```rust
use chatwork::{v2::rooms::messages, Client};

let client = Client::new(api_token);
let svc = client.v2().rooms().messages(room_id);
let message_id = svc.create(&messages::Create { body: "hello", ..Default::default() }).await?;
println!("message_id: {:?}", message_id);
```

## API

| Method | Path                                    | Client                                                       |
|--------|-----------------------------------------|--------------------------------------------------------------|
| GET    | /me                                     | c.v2().me().get()                                            |
| GET    | /my/status                              | c.v2().my().status().get()                                   |
| GET    | /my/tasks                               | c.v2().my().tasks().list(list)                               |
| GET    | /contacts                               | c.v2().contacts().list()                                     |
| GET    | /rooms                                  | c.v2().rooms().list()                                        |
| POST   | /rooms                                  | c.v2().rooms().create(create)                                |
| GET    | /rooms/{room_id}                        | c.v2().rooms().get(room_id)                                  |
| PUT    | /rooms/{room_id}                        | c.v2().rooms().update(room_id, update)                       |
| DELETE | /rooms/{room_id}                        | c.v2().rooms().delete(room_id, delete)                       |
| GET    | /rooms/{room_id}/members                | c.v2().rooms().members(room_id).list()                       |
| PUT    | /rooms/{room_id}/members                | c.v2().rooms().members(room_id).update(update)               |
| GET    | /rooms/{room_id}/messages               | c.v2().rooms().messages(room_id).list(list)                  |
| POST   | /rooms/{room_id}/messages               | c.v2().rooms().messages(room_id).create(create)              |
| GET    | /rooms/{room_id}/messages/{message_id}  | c.v2().rooms().messages(room_id).get(message_id)             |
| PUT    | /rooms/{room_id}/messages/{message_id}  | c.v2().rooms().messages(room_id).update(message_id, update)  |
| DELETE | /rooms/{room_id}/messages/{message_id}  | c.v2().rooms().messages(room_id).delete(message_id)          |
| PUT    | /rooms/{room_id}/messages/read          | c.v2().rooms().messages(room_id).read().update(update)       |
| PUT    | /rooms/{room_id}/messages/unread        | c.v2().rooms().messages(room_id).unread().update(update)     |
| GET    | /rooms/{room_id}/tasks                  | c.v2().rooms().tasks(room_id).list(list)                     |
| POST   | /rooms/{room_id}/tasks                  | c.v2().rooms().tasks(room_id).create(create)                 |
| GET    | /rooms/{room_id}/tasks/{task_id}        | c.v2().rooms().tasks(room_id).get(task_id)                   |
| PUT    | /rooms/{room_id}/tasks/{task_id}/status | c.v2().rooms().tasks(room_id).status(task_id).update(update) |
| GET    | /rooms/{room_id}/files                  | c.v2().rooms().files(room_id).list(list)                     |
| POST   | /rooms/{room_id}/files                  | c.v2().rooms().files(room_id).create(create)                 |
| GET    | /rooms/{room_id}/files/{file_id}        | c.v2().rooms().files(room_id).get(file_id, get)              |
| GET    | /rooms/{room_id}/link                   | c.v2().rooms().link(room_id).get()                           |
| POST   | /rooms/{room_id}/link                   | c.v2().rooms().link(room_id).create(create)                  |
| PUT    | /rooms/{room_id}/link                   | c.v2().rooms().link(room_id).update(update)                  |
| DELETE | /rooms/{room_id}/link                   | c.v2().rooms().link(room_id).delete()                        |
| GET    | /incoming_requests                      | c.v2().incoming_requests().list()                            |
| PUT    | /incoming_requests/{request_id}         | c.v2().incoming_requests().update(request_id)                |
| DELETE | /incoming_requests/{request_id}         | c.v2().incoming_requests().delete(request_id)                |


## License

Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.
