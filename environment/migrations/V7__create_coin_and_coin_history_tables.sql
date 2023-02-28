CREATE table coin
(
    id     varchar(50) PRIMARY KEY,
    symbol varchar(50),
    name   varchar(255)
);

CREATE table coin_history
(
    id        uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol    varchar(50),
    btc       decimal,
    eth       decimal,
    eur       decimal,
    jpy       decimal,
    usd       decimal,
    datestamp date
);