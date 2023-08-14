ALTER TABLE asset ADD COLUMN attribute VARCHAR;
ALTER TABLE asset DROP COLUMN name;

UPDATE asset
SET attribute = CASE
                    WHEN token_address = '0x9e0d99b864e1ac12565125c5a82b59adea5a09cd' THEN concat('Tier ', metadata->>'tier')
                    WHEN token_address = '0xc1f1da534e227489d617cd742481fd5a23f6a003' THEN concat(metadata->>'name',
                                                                                                  CASE
                                                                                                      WHEN (metadata->>'Alpha')::boolean THEN ' Alpha'
                                                                                                      END, ' Wave ', metadata->>'Wave')
                    WHEN token_address = '0x8cceea8cfb0f8670f4de3a6cd2152925605d19a8' THEN concat('Set ', metadata->>'Set', ' Wave ', metadata->>'Wave', ' Tier ', metadata->>'Tier',
                                                                                                  CASE
                                                                                                      WHEN (metadata->>'Finish' = 'Holo') THEN ' Holo'
                                                                                                      END)
                    WHEN token_address = '0x844a2a2b4c139815c1da4bdd447ab558bb9a7d24' THEN concat('Set ', metadata->>'Set', ' Tier ', metadata->>'Tier',
                                                                                                  ' Stage ', metadata->>'Stage')
    END;

CREATE OR REPLACE FUNCTION update_asset_attribute()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.attribute = CASE
                        WHEN NEW.token_address = '0x9e0d99b864e1ac12565125c5a82b59adea5a09cd' THEN concat('Tier ', NEW.metadata->>'tier')
                        WHEN NEW.token_address = '0xc1f1da534e227489d617cd742481fd5a23f6a003' THEN concat(NEW.metadata->>'name',
                                                                                                          CASE
                                                                                                              WHEN (NEW.metadata->>'Alpha')::boolean THEN ' Alpha'
                                                                                                              END, ' Wave ', NEW.metadata->>'Wave')
                        WHEN NEW.token_address = '0x8cceea8cfb0f8670f4de3a6cd2152925605d19a8' THEN concat('Set ', NEW.metadata->>'Set', ' Wave ', NEW.metadata->>'Wave', ' Tier ', NEW.metadata->>'Tier',
                                                                                                          CASE
                                                                                                              WHEN (NEW.metadata->>'Finish' = 'Holo') THEN ' Holo'
                                                                                                              END)
                        WHEN NEW.token_address = '0x844a2a2b4c139815c1da4bdd447ab558bb9a7d24' THEN concat('Set ', NEW.metadata->>'Set', ' Tier ', NEW.metadata->>'Tier',
                                                                                                          ' Stage ', NEW.metadata->>'Stage')
        END;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_attribute_trigger
    BEFORE INSERT OR UPDATE ON asset
    FOR EACH ROW
EXECUTE FUNCTION update_asset_attribute();

drop materialized view total_minted_and_burnt_by_attribute_mat_view;
drop materialized view cheapest_and_most_expensive_trades_by_attribute_mat_view;
drop materialized view floor_data_mat_by_attribute_view;

create materialized view cheapest_and_most_expensive_trades_by_attribute_mat_view as
select attribute, token_id, token_address, name, sum_usd, buy_currency, buy_price, wallet_to, wallet_from, updated_on, transaction_id
from (
         select od.token_address,
                a.token_id,
                a.attribute,
                a.metadata->>'name' as name,
                round((od.buy_price * ch.usd), 2) as sum_usd,
                od.buy_currency,
                od.buy_price,
                od.wallet_to,
                od.wallet_from,
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
    t.name, t.attribute, t.token_id, t.buy_price, t.buy_currency, t.token_address
FROM
    (
        SELECT
                a.metadata->>'name' as name, a.attribute, o.token_id, o.token_address, o.buy_price, o.buy_currency, row_number() OVER (
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

create index a_attribute_index on asset (attribute);