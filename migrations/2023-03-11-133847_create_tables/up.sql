-- Your SQL goes here

CREATE TABLE IF NOT EXISTS deaneries (
  id SERIAL PRIMARY KEY,
  number VARCHAR(10) NOT NULL,
  name VARCHAR(255) NOT NULL
);


CREATE TABLE IF NOT EXISTS subjects (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  deanery_id integer REFERENCES deaneries(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS groups (
  id SERIAL PRIMARY KEY,
  number VARCHAR(10) NOT NULL,
  deanery_id integer REFERENCES deaneries(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  fio VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  phone VARCHAR(12) NOT NULL,
  position_office VARCHAR(255),
  group_id integer REFERENCES groups(id),
  is_teacher BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS teacher_deanery(
  id SERIAL PRIMARY KEY,
  teacher_id integer REFERENCES users(id) NOT NULL,
  deanery_id integer REFERENCES deaneries(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS group_subject (
  id SERIAL PRIMARY KEY,
  group_id integer REFERENCES groups(id) NOT NULL,
  subject_id integer REFERENCES subjects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS teacher_subject (
  id SERIAL PRIMARY KEY,
  teacher_id integer REFERENCES users(id) NOT NULL,
  subject_id integer REFERENCES subjects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS chats (
  id SERIAL NOT NULL PRIMARY KEY,
  title VARCHAR(60) NOT NULL,
  is_dialog BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS content_message (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  type_content TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS discussions (
  id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS messages (
  id SERIAL PRIMARY KEY,
  chat_id integer REFERENCES chats(id),
  discussion_id integer REFERENCES discussions(id),
  sender_id integer REFERENCES users(id) NOT NULL,
  date_send TIMESTAMP NOT NULL,
  content_id integer REFERENCES content_message(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS users_chats (
    user_id int REFERENCES users(id),
    chat_id int REFERENCES chats(id),
    PRIMARY KEY(user_id, chat_id)
);