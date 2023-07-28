ALTER TABLE asset ADD COLUMN metadata jsonb;
UPDATE asset SET metadata = jsonb_build_object(
        'name', name,
        'tier', tier,
        'solon', solon,
        'carbon', carbon,
        'crypton', crypton,
        'silicon', silicon,
        'hydrogen', hydrogen,
        'hyperion', hyperion,
        'landmark', landmark
);

drop materialized view cheapest_and_most_expensive_trades_by_tier_mat_view;

ALTER TABLE asset DROP COLUMN tier, DROP COLUMN solon,
                  DROP COLUMN carbon, DROP COLUMN crypton, DROP COLUMN silicon,
                  DROP COLUMN hydrogen, DROP COLUMN hyperion, DROP COLUMN landmark;

create materialized view cheapest_and_most_expensive_trades_by_attribute_mat_view as
WITH cte AS (
    SELECT
        a.token_id,
        a.token_address,
        a.name,
        CASE
            WHEN a.token_address = '0x9e0d99b864e1ac12565125c5a82b59adea5a09cd' THEN concat('Tier ', a.metadata->>'tier')
            WHEN a.token_address = '0xc1f1da534e227489d617cd742481fd5a23f6a003' THEN concat(a.name,
                                                                                            CASE
                                                                                                WHEN (a.metadata->>'Alpha')::boolean THEN ' Alpha'
                                                                                                END, ' Wave ', a.metadata->>'Wave')
            END AS attribute
    FROM
        asset a
)
select attribute, token_id, token_address, name, sum_usd, buy_currency, buy_price, wallet_to, wallet_from, updated_on, transaction_id
from (
         select od.token_address,
                cte.token_id,
                cte.attribute,
                cte.name,
                round((od.buy_price * ch.usd), 2) as sum_usd,
                od.buy_currency,
                od.buy_price,
                od.wallet_to,
                od.wallet_from,
                od.updated_on,
                od.transaction_id,
                row_number() over (partition by cte.attribute, od.token_address order by (od.buy_price * ch.usd) desc) as highest_rn,
                row_number() over (partition by cte.attribute, od.token_address order by (od.buy_price * ch.usd)) as lowest_rn
         from cte
                  join order_data od on cte.token_id = od.token_id and cte.token_address = od.token_address
                  join coin_history ch on ch.datestamp = od.updated_on::date and od.buy_currency = ch.symbol
         where od.status = 'filled'
     ) subquery
where highest_rn = 1 or lowest_rn = 1
order by token_address, attribute, sum_usd;

create materialized view floor_data_mat_view as
WITH cte AS (
    SELECT
        a.token_id,
        a.token_address,
        a.name,
        CASE
            WHEN a.token_address = '0x9e0d99b864e1ac12565125c5a82b59adea5a09cd' THEN concat('Tier ', a.metadata->>'tier')
            WHEN a.token_address = '0xc1f1da534e227489d617cd742481fd5a23f6a003' THEN concat(a.name,
                                                                                            CASE
                                                                                                WHEN (a.metadata->>'Alpha')::boolean THEN ' Alpha'
                                                                                                END, ' Wave ', a.metadata->>'Wave')
            END AS attribute
    FROM
        asset a
)
SELECT
    t.name, t.attribute, t.token_id, t.buy_price, t.buy_currency, t.token_address
FROM
    (
        SELECT
            cte.name, cte.attribute, o.token_id, o.token_address, o.buy_price, o.buy_currency, row_number() OVER (
            PARTITION BY
                cte.attribute,
                o.buy_currency
            ORDER BY
                o.buy_price
            ) AS rn
        FROM
            cte
                INNER JOIN (
                SELECT
                    token_id,
                    token_address,
                    buy_currency,
                    MIN(buy_price) AS buy_price
                FROM
                    order_data
                WHERE
                        status = 'active'
                GROUP BY
                    token_id,
                    token_address,
                    buy_currency
            ) o ON cte.token_id = o.token_id
                AND cte.token_address = o.token_address
    ) t WHERE rn = 1 ORDER BY attribute, buy_price;

create materialized view total_minted_and_burnt_mat_view as
select CASE
           WHEN token_address = '0x9e0d99b864e1ac12565125c5a82b59adea5a09cd' THEN concat('Tier ', metadata->>'tier')
           WHEN token_address = '0xc1f1da534e227489d617cd742481fd5a23f6a003' THEN concat(name,
                                                                                         CASE
                                                                                             WHEN (metadata->>'Alpha')::boolean THEN ' Alpha'
                                                                                             END, ' Wave ', metadata->>'Wave')
           end as attribute,
       count(*) as total_minted,
       count(*) filter (where current_owner = '0x0000000000000000000000000000000000000000') as total_burnt,
       token_address from asset
group by attribute, token_address
order by attribute;

ALTER TABLE collection ADD COLUMN enabled boolean default false;

ALTER TABLE asset
    DROP CONSTRAINT asset_pkey;
ALTER TABLE asset
    ADD CONSTRAINT pk_asset PRIMARY KEY (token_id, token_address);
drop index a_token_address_index;

drop materialized view asset_current_owner_mat_view;

create materialized view asset_current_owner_mat_view as
select count(*) as total,
       COUNT(*) FILTER (WHERE current_owner = '0x0000000000000000000000000000000000000000') AS total_burnt,
       count(distinct (current_owner)) as total_owners,
       token_address from asset
group by token_address;