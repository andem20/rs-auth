db = db.getSiblingDB('auth');

db.createCollection('users');
db.createCollection('tokens');

db.users.insertMany([
 {
    username: 'fenrir',
    password: '$2b$12$xwYQPeJzoL83jWaDnJuqiui9wpCj6Wjd5LmDSwdi/Uk4pjq4.ta1C',
  },
  {
    username: 'aviaia',
    password: '$2b$12$fjxHYBQjgJeozw5itejU/eX.4PQWFBBW9yHHY0db9kns04yx6Xg1C'
  },
]);