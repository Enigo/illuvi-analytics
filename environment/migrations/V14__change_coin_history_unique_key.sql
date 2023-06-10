ALTER TABLE coin_history
    DROP CONSTRAINT coin_history_pkey;
ALTER TABLE coin_history
    DROP COLUMN id;

ALTER TABLE coin_history
    ADD CONSTRAINT pk_coin_history PRIMARY KEY (symbol, datestamp);
