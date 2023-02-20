CREATE table transfer
(
    transaction_id integer PRIMARY KEY,
    status         varchar(50),
    wallet_from    varchar(255),
    wallet_to      varchar(255),
    token_id       integer,
    token_address  varchar(255),
    created_on     timestamp
);