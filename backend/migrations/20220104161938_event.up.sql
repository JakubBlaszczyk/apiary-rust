-- Add up migration script here
CREATE TABLE EVENT (
  ID              SERIAL NOT NULL, 
  ID_apiary       int4 NOT NULL, 
  Time_start      timestamp, 
  Time_end        timestamp, 
  note            varchar(1023), 
  PRIMARY KEY (ID));
ALTER TABLE EVENT ADD CONSTRAINT FKEVENT402333 FOREIGN KEY (ID_apiary) REFERENCES APIARY (ID);
INSERT INTO EVENT (ID_apiary, Time_start, note) VALUES (1, CURRENT_TIMESTAMP, 'There was event');
INSERT INTO EVENT (ID_apiary, Time_start, note) VALUES (3, CURRENT_TIMESTAMP, 'Bear attacked!');