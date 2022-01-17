CREATE TABLE ACCOUNT (
  ID    UUID NOT NULL DEFAULT gen_random_uuid(),
  LOGIN         varchar(255) NOT NULL,
  PASSWORD    varchar(255) NOT NULL,
  EMAIL       varchar(255) NOT NULL,
  PRIVILEGES varchar(255) NOT NULL,
  CONSTRAINT ID
    PRIMARY KEY (ID));
INSERT INTO ACCOUNT (LOGIN, PASSWORD, EMAIL, PRIVILEGES) VALUES ('admin', 'admin', 'admin@admin.com', 'admin');
INSERT INTO ACCOUNT (LOGIN, PASSWORD, EMAIL, PRIVILEGES) VALUES ('user1', 'user1', 'user1@user1.com', 'worker');
INSERT INTO ACCOUNT (LOGIN, PASSWORD, EMAIL, PRIVILEGES) VALUES ('user2', 'user2', 'user2@user2.com', 'beekeeper');
INSERT INTO ACCOUNT (LOGIN, PASSWORD, EMAIL, PRIVILEGES) VALUES ('user3', 'user3', 'user3@user3.com', 'worker');