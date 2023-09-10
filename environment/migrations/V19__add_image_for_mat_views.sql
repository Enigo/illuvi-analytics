drop materialized view cheapest_and_most_expensive_trades_by_attribute_mat_view;
drop materialized view floor_data_mat_by_attribute_view;

create materialized view cheapest_and_most_expensive_trades_by_attribute_mat_view as
select attribute, token_id, token_address, name, image_url, sum_usd, buy_currency, buy_price, updated_on, transaction_id
from (
         select od.token_address,
                a.token_id,
                a.attribute,
                a.metadata->>'name' as name,
                a.metadata->>'image_url' as image_url,
                round((od.buy_price * ch.usd), 2) as sum_usd,
                od.buy_currency,
                od.buy_price,
                od.updated_on,
                od.transaction_id,
                row_number() over (partition by a.attribute, od.token_address order by (od.buy_price * ch.usd) desc) as highest_rn,
                row_number() over (partition by a.attribute, od.token_address order by (od.buy_price * ch.usd)) as lowest_rn
         from asset a
                  join order_data od on a.token_id = od.token_id and a.token_address = od.token_address
                  join coin_history ch on ch.datestamp = od.updated_on::date and od.buy_currency = ch.symbol
         where od.status = 'filled'
     ) subquery
where highest_rn = 1 or lowest_rn = 1
order by token_address, attribute, sum_usd;

create materialized view floor_data_mat_by_attribute_view as
SELECT
    t.name, t.image_url, t.attribute, t.token_id, t.buy_price, t.buy_currency, t.token_address
FROM
    (
        SELECT
                a.metadata->>'name' as name, a.metadata->>'image_url' as image_url, a.attribute, o.token_id,
                o.token_address, o.buy_price, o.buy_currency, row_number() OVER (
            PARTITION BY
                a.attribute,
                o.buy_currency
            ORDER BY
                o.buy_price
            ) AS rn
        FROM
            asset a
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
            ) o ON a.token_id = o.token_id
                AND a.token_address = o.token_address
        where a.metadata->>'name' is not null
    ) t WHERE rn = 1 ORDER BY attribute, buy_price;