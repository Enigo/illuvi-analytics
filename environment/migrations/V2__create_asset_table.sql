CREATE table asset
(
    token_id      integer PRIMARY KEY,
    token_address varchar(255),
    name          varchar(50),
    tier          integer,
    solon         integer,
    carbon        integer,
    crypton       integer,
    silicon       integer,
    hydrogen      integer,
    hyperion      integer,
    landmark      varchar(50)
);