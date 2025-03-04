db = db.getSiblingDB("main");

db.createCollection("users");

db.createCollection("messages");

db.users.insertOne({
  login: "admin",
  password: "admin123",
});

db.messages.insertOne({
  sender: "admin",
  recipient: "user1",
  content: "Welcome to the chat!",
  timestamp: NumberLong(Math.floor(Date.now() / 1000)),
});
