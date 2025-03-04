# chat-app
It's web application with HTTP requests and Web-Sockets, it allows to register user and login user in account, after authorization user will get JWT token. With JWT token user can get chat history and connect to websocket for online chatting.

## Technologies
- [Rust](https://www.rust-lang.org/ru)
- [Actix Web](https://actix.rs/)
- [MongoDB](https://www.mongodb.com/)
- [Docker](https://www.docker.com/)
- [Sentry](https://sentry.io/welcome/)

## Usage
### Registration of user
Post request

    https://saniray.kz/auth/registration
With body 

    {
        "login": "<Your login>",
        "password": "<Your Password>"
    }

After successful registration user has the opportunity to login to his account
### Authorization of user
Post request

    https://saniray.kz/auth/login
With body 

    {
        "login": "<Your login>",
        "password": "<Your Password>"
    }
After successful authorization user will get his JWT token, which is available only during 1 hour

Response example:

    {
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJhYSIsImV4cCI6MTc0MTExNDQ2Nn0.UehpmVybfx8HavHkPE0g_ZnssjJHWxkGcwBLUZcOPUM"
    }

### Websocket connection
For online chatting user must indicate get request to websocket with JWT token

    ws://saniray.kz/messages/ws?token=<JWT token>

After successfully connecting to the websocket, the user can send a message to an existing user in the database

    {
        "sender": "<Sender login>",
        "recipient": "<Recipient login>",
        "content": "Example Message"
    }

If the recipient is also connected to the web socket, he will receive the message, and during sending, all messages are saved to the database

### Getting all messages
Also user has the opportunity to read all history of messages
Get request

    https://saniray.kz//messages/old_messages

With header

    Authorization: Bearer <JWT token>
Example response:

    [
        {
            "sender": "azarenkov",
            "recipient": "aa",
            "content": "Hello from Postman!",
            "timestamp": 1741110925
        },
        {
            "sender": "aa",
            "recipient": "azarenkov",
            "content": "Hello from Postman!",
            "timestamp": 1741110932
        },
        {
            "sender": "aa",
            "recipient": "azarenkov",
            "content": "Hello from Postman!",
            "timestamp": 1741110943
        }
    ]


## Middleware
For middleware have been chosen Sentry. There developers can find logs about all activities, which are connected with HTTP requests and websockets
![sentry1](assets/sentry1.png)
![sentry2](assets/sentry2.png)

## Local Usage
If you need to run this programm local, you should to create .env file in the main directory with

    MONGODB_URI=mongodb://root:example@localhost:27017
    JWT_SECRET=<Your secret words>
    PORT=<Your port>
    SENTRY=<Your Sentry URL>

And also use this command for creating MongoDB, it will call script which will configure your database

    docker-compose up -d

After these actions, you can run the application

    cargo run --release


## Developers
Contacts
- [Alexey Azarenkov](https://t.me/azarenkov_alexey) — Rust Developer
