-- Add up migration script here
CREATE TABLE APIARY (
  ID    SERIAL NOT NULL PRIMARY KEY,
  LOCALIZATION           varchar(255),
  INFORMATION varchar(255));