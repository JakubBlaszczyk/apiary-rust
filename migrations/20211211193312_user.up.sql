-- Add up migration script here
CREATE TABLE ACCOUNT (
  ID    UUID NOT NULL DEFAULT gen_random_uuid(),
  LOGIN         varchar(255) NOT NULL,
  PASSWORD    varchar(255) NOT NULL,
  EMAIL       varchar(255) NOT NULL,
  PRIVILEGES varchar(255) NOT NULL,
  CONSTRAINT ID
    PRIMARY KEY (ID));