# chat-app
It's web application with HTTP requests and Web-Sockets, it allows to register user and login user in account, after authorization user will get JWT token. With JWT token user can get chat history and connect to websocket for online chatting.

## Technologies
- [Rust](https://www.rust-lang.org/ru)
- [Actix Web](https://actix.rs/)
- [MongoDB](https://www.mongodb.com/)
- [Docker](https://www.docker.com/)

## Usage
### Registration of user
Post request

    https://saniray.kz/auth/registration
With body 

    {
        "login": "<Your login>",
        "password": "<Your Password>"
    }

After successful registration user has opportunity to login to his account
### Authorization of user
Post request

    https://saniray.kz/auth/login
With body 

    {
        "login": "<Your login>",
        "password": "<Your Password>"
    }
After successful authorization user will get his JWT token, which is available only during 1 hour

Response
    todo

## Developers
Contacts
- [Alexey Azarenkov](https://t.me/azarenkov_alexey) — Rust Developer
