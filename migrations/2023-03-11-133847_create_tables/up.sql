-- Your SQL goes here

CREATE TABLE IF NOT EXISTS administration (
  id SERIAL PRIMARY KEY,
  fio VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  phone VARCHAR(12) NOT NULL,
  position_office VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS sections (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL
);


CREATE TABLE IF NOT EXISTS workers (
  id SERIAL PRIMARY KEY,
  fio VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  phone VARCHAR(12) NOT NULL,
  position_office VARCHAR(255) NOT NULL,
  section_id integer REFERENCES sections(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS deaneries (
  id SERIAL PRIMARY KEY,
  number VARCHAR(10) NOT NULL,
  name VARCHAR(255) NOT NULL
);


CREATE TABLE IF NOT EXISTS departments (
  id SERIAL PRIMARY KEY,
  number VARCHAR(10) NOT NULL,
  name VARCHAR(255) NOT NULL,
  deanery_id integer REFERENCES deaneries(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS subjects (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  department_id integer REFERENCES departments(id) NOT NULL
);


CREATE TABLE IF NOT EXISTS teachers (
  id SERIAL PRIMARY KEY,
  fio VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  phone VARCHAR(12) NOT NULL
);


CREATE TABLE IF NOT EXISTS teacher_department (
  id SERIAL PRIMARY KEY,
  teacher_id integer REFERENCES teachers(id) NOT NULL,
  department_id integer REFERENCES departments(id) NOT NULL
);


CREATE TABLE IF NOT EXISTS groups (
  id SERIAL PRIMARY KEY,
  number VARCHAR(10) NOT NULL,
  department_id integer REFERENCES departments(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS group_subject (
  id SERIAL PRIMARY KEY,
  group_id integer REFERENCES groups(id) NOT NULL,
  subject_id integer REFERENCES subjects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS teacher_subject (
  id SERIAL PRIMARY KEY,
  teacher_id integer REFERENCES teachers(id) NOT NULL,
  subject_id integer REFERENCES subjects(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS students (
  id SERIAL PRIMARY KEY,
  fio VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  phone VARCHAR(12) NOT NULL,
  group_id integer REFERENCES groups(id) NOT NULL
);


CREATE TABLE IF NOT EXISTS chats (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS content_message (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  type_content TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
  id SERIAL PRIMARY KEY,
  chat_id integer REFERENCES chats(id) NOT NULL,
  sender_type VARCHAR(255) NOT NULL,
  sender_id integer NOT NULL,
  date_send TIMESTAMP NOT NULL,
  content_id integer REFERENCES content_message(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS p_to_p (
  id SERIAL PRIMARY KEY,
  chat_id integer REFERENCES chats(id) NOT NULL,
  user_id integer NOT NULL,
  type_user VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS t_to_g (
  id SERIAL PRIMARY KEY,
  chat_id integer REFERENCES chats(id) NOT NULL,
  admin integer REFERENCES teachers(id) NOT NULL,
  group_id integer REFERENCES groups(id) NOT NULL
);

CREATE TABLE IF NOT EXISTS users_many (
  id SERIAL PRIMARY KEY,
  chat_id integer REFERENCES chats(id) NOT NULL,
  user_id integer NOT NULL,
  type_user VARCHAR(255) NOT NULL
);
