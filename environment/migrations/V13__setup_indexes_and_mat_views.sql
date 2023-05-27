create index a_token_address_index on asset (token_address);
create index a_current_owner_index on asset (current_owner);

create index ch_symbol_datestamp_index on coin_history (symbol, datestamp);

create index m_token_id_index on mint (token_id);
create index m_token_address_index on mint (token_address);

create index od_token_id_index on order_data (token_id);
create index od_token_address_index on order_data (token_address);
create index od_status_index on order_data (status);

create index t_token_id_index on transfer (token_id);
create index t_token_address_index on transfer (token_address);

create materialized view asset_current_owner_mat_view as
select count(*) as total, count(distinct (current_owner)) as total_owners, token_address from asset
group by token_address;

create materialized view trade_volume_mat_view as
select od.token_address,
       round(sum(od.buy_price * ch.eth), 2) as sum_eth,
       round(sum(od.buy_price * ch.usd), 2) as sum_usd
from order_data od
         join coin_history ch on od.buy_currency = ch.symbol and ch.datestamp = od.updated_on::date
where od.status = 'filled'
group by od.token_address;

create materialized view cheapest_and_most_expensive_trades_by_tier_mat_view as
select tier, token_id, token_address, name, sum_usd,  buy_currency, buy_price, wallet_to, wallet_from, updated_on, transaction_id
from (
         select od.token_address,
                a.token_id,
                a.tier,
                a.name,
                round((od.buy_price * ch.usd), 2) as sum_usd,
                od.buy_currency,
                od.buy_price,
                od.wallet_to,
                od.wallet_from,
                od.updated_on,
                od.transaction_id,
                row_number() over (partition by a.tier, od.token_address order by (od.buy_price * ch.usd) desc) as highest_rn,
                row_number() over (partition by a.tier, od.token_address order by (od.buy_price * ch.usd)) as lowest_rn
         from asset a
                  join order_data od on a.token_id = od.token_id
                  join coin_history ch on ch.datestamp = od.updated_on::date and od.buy_currency = ch.symbol
         where od.status = 'filled'
     ) subquery
where highest_rn = 1 or lowest_rn = 1
order by token_address, tier, sum_usd;

create materialized view trade_volume_full_mat_view as
select count(*) as total_trades,
       round(SUM(od.buy_price), 6) as total_in_buy_currency,
       od.buy_currency,
       od.token_address,
       round(SUM(od.buy_price * ch.btc), 6) as total_btc,
       round(SUM(od.buy_price * ch.eth), 6) as total_eth,
       round(SUM(od.buy_price * ch.usd), 6) as total_usd,
       round(SUM(od.buy_price * ch.eur), 6) as total_eur,
       round(SUM(od.buy_price * ch.jpy), 6) as total_jpy
from order_data od join coin_history ch on od.buy_currency = ch.symbol AND ch.datestamp = od.updated_on::DATE
where od.status='filled'
group by od.buy_currency, od.token_address
order by total_usd desc;