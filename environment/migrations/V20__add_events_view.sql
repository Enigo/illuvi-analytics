create view events_view as
with max_date as (
    select max(datestamp) as max_datestamp from coin_history)
select *
from (
         select transaction_id,
                case
                    when wallet_to='0x0000000000000000000000000000000000000000'
                        then 'Burned'
                    else 'Transfer' end as event, token_address, token_id, wallet_from,
                case
                    when wallet_to='0x0000000000000000000000000000000000000000'
                        then null
                    else wallet_to end as wallet_to, created_on as timestamp, null as currency, null as price, null as usd_price
         from transfer
         union all
         select od.transaction_id, concat('Trade ', od.status) as event, od.token_address, od.token_id, od.wallet_from, od.wallet_to, od.updated_on as timestamp,
                od.buy_currency as currency, od.buy_price as price, round(od.buy_price * ch.usd, 2) as usd_price
         from order_data od
                  left join coin_history ch on ch.datestamp = case when od.status = 'active' then
                                                                       (select max_date.max_datestamp from max_date)
                                                                   else od.updated_on::date end
             and od.buy_currency = ch.symbol
         union all
         select m.transaction_id, 'Mint' as event, token_address, token_id, null as wallet_from, m.wallet as wallet_to, m.minted_on as timestamp, m.currency, m.price, round((m.price * ch.usd), 2) as usd_price
         from mint m
                  left join coin_history ch on ch.datestamp = m.minted_on::date and ch.symbol = m.currency
         union all
         select transaction_id, 'Deposit' as event, token_address, token_id, null as wallet_from, wallet as wallet_to, created_on as timestamp, null as currency, null as price, null as usd_price
         from deposit
         union all
         select transaction_id, 'Withdrawal' as event, token_address, token_id, wallet as wallet_from, null as wallet_to, created_on as timestamp, null as currency, null as price, null as usd_price
         from withdrawal
     ) as combined_events;

drop index if exists ch_symbol_datestamp_index;