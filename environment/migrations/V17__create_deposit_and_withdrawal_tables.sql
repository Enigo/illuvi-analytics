CREATE table deposit
(
    transaction_id integer PRIMARY KEY,
    status         varchar(50),
    wallet         varchar(255),
    token_id       integer,
    token_address  varchar(255),
    created_on     timestamp
);

CREATE table withdrawal
(
    transaction_id integer PRIMARY KEY,
    status         varchar(50),
    wallet         varchar(255),
    token_id       integer,
    token_address  varchar(255),
    created_on     timestamp
);

ALTER TABLE mint DROP COLUMN token_type;