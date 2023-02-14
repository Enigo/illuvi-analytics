-- order is a reserved keyword and I don't want to use plural case
CREATE table order_data
(
    order_id      integer PRIMARY KEY,
    status        varchar(50),
    wallet_from   varchar(255),
    token_id      integer,
    token_address varchar(255),
    buy_currency  varchar(50),
    buy_price     decimal,
    created_on    timestamp,
    updated_on    timestamp
);