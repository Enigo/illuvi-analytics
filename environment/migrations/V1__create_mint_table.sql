CREATE table mint
(
    transaction_id integer PRIMARY KEY,
    status varchar(50),
    wallet  varchar(255),
    token_type  varchar(15),
    token_id  varchar(15),
    minted_on timestamp
);