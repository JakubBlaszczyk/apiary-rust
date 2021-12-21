-- Add up migration script here
CREATE TABLE Account (
  id    UUID NOT NULL DEFAULT gen_random_uuid(),
  login         varchar(255) NOT NULL,
  password    varchar(255) NOT NULL,
  email       varchar(255) NOT NULL,
  privileges varchar(255) NOT NULL,
  CONSTRAINT id
    PRIMARY KEY (id));