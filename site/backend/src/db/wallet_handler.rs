use log::error;
use model::model::price::Price;
use model::model::wallet::{TotalPerCollectionData, WalletData, WalletMoneyData};
use sqlx::types::Decimal;
use sqlx::{query_as, FromRow, Pool, Postgres};

pub async fn get_wallet(pool: &Pool<Postgres>, wallet: &String) -> Option<WalletData> {
    let money_data = get_money_data(pool, wallet).await;
    let minted_per_collection_wallet = get_minted_per_collection_wallet(pool, wallet).await;
    let owned_per_collection_wallet = get_owned_per_collection_wallet(pool, wallet).await;

    return Some(WalletData {
        wallet: wallet.clone(),
        minted_per_collection_wallet,
        owned_per_collection_wallet,
        money_data,
    });
}

async fn get_minted_per_collection_wallet(
    pool: &Pool<Postgres>,
    wallet: &String,
) -> Vec<TotalPerCollectionData> {
    return match query_as::<_, TotalPerCollectionDb>(
        "SELECT
            c.name,
            COALESCE(mint_counts.count_mints, 0) AS total_per_wallet
        FROM collection c
                 LEFT JOIN (
            SELECT
                c.name AS name,
                COUNT(*) AS count_mints
            FROM mint m
                     INNER JOIN collection c ON m.token_address = c.address
            where m.wallet=$1
            GROUP BY c.name
        ) AS mint_counts ON c.name = mint_counts.name
        ORDER BY c.name",
    )
    .bind(wallet)
    .fetch_all(pool)
    .await
    {
        Ok(result) => result.into_iter().map(|value| value.into()).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn get_owned_per_collection_wallet(
    pool: &Pool<Postgres>,
    wallet: &String,
) -> Vec<TotalPerCollectionData> {
    return match query_as::<_, TotalPerCollectionDb>(
        "SELECT
            c.name,
            COALESCE(assets_counts.total, 0) AS total_per_wallet
        FROM collection c
                 LEFT JOIN (
            SELECT
                c.name,
                COUNT(*) AS total
            FROM asset a
                     INNER JOIN collection c ON a.token_address = c.address
            where a.current_owner=$1
            GROUP BY c.name
        ) AS assets_counts ON c.name = assets_counts.name
        ORDER BY c.name",
    )
    .bind(wallet)
    .fetch_all(pool)
    .await
    {
        Ok(result) => result.into_iter().map(|value| value.into()).collect(),
        Err(e) => {
            error!("Error fetching data: {e}");
            vec![]
        }
    };
}

async fn get_money_data(pool: &Pool<Postgres>, wallet: &String) -> WalletMoneyData {
    let zero_price = Price {
        price: 0_f64,
        currency: String::from("USD"),
    };
    let mint_spend_usd = match query_as::<_, TotalDb>(
        "select sum(round((m.price * ch.usd), 2)) as total_usd, count(*) as total from mint m
             join coin_history ch on ch.datestamp = m.minted_on::date and ch.symbol = m.currency
         where m.wallet=$1
         and m.price is not null",
    )
    .bind(wallet)
    .fetch_one(pool)
    .await
    {
        Ok(result) => result.total_usd.map_or(zero_price.clone(), |value| Price {
            price: f64::try_from(value).unwrap(),
            currency: String::from("USD"),
        }),
        Err(e) => {
            error!("Error fetching data: {e}");
            zero_price.clone()
        }
    };

    let filled_data = match query_as::<_, FilledOrderDb>(
        "select
            sum(round((od.buy_price * ch.usd), 2)) filter ( where od.wallet_from=$1 and od.status='filled') as total_sell_usd,
            sum(round((od.buy_price * ch.usd), 2)) filter ( where od.wallet_to=$1 and od.status='filled') as total_buy_usd
         from order_data od join coin_history ch on ch.datestamp = od.updated_on::date and ch.symbol = od.buy_currency")
        .bind(wallet)
        .fetch_one(pool)
        .await
    {
        Ok(result) => {
            (result.total_sell_usd.map_or(zero_price.clone(), |value| Price {
                price: f64::try_from(value).unwrap(),
                currency: String::from("USD"),
            }), result.total_buy_usd.map_or(zero_price.clone(), |value| Price {
                price: f64::try_from(value).unwrap(),
                currency: String::from("USD"),
            }))
        },
        Err(e) => {
            error!("Error fetching data: {e}");
            (zero_price.clone(), zero_price.clone())
        }
    };

    let total_active = match query_as::<_, TotalDb>(
        "select sum(round((od.buy_price * ch.usd), 2)) as total_usd, count(*) as total
            from order_data od join coin_history ch on ch.datestamp = (select max(datestamp) from coin_history) and ch.symbol = od.buy_currency
            where od.status='active' and od.wallet_from=$1")
        .bind(wallet)
        .fetch_one(pool)
        .await
    {
        Ok(result) => result.total_usd.map_or((zero_price.clone(), 0), |value| (Price {
            price: f64::try_from(value).unwrap(),
            currency: String::from("USD"),
        }, result.total)),
        Err(e) => {
            error!("Error fetching data: {e}");
            (zero_price, 0)
        }
    };

    return WalletMoneyData {
        mint_spend_usd,
        total_sell_usd: filled_data.0,
        total_buy_usd: filled_data.1,
        total_active_usd: total_active.0,
        total_active: total_active.1,
    };
}

#[derive(FromRow)]
struct TotalDb {
    total_usd: Option<Decimal>,
    total: i64,
}

#[derive(FromRow)]
struct FilledOrderDb {
    total_sell_usd: Option<Decimal>,
    total_buy_usd: Option<Decimal>,
}

#[derive(FromRow)]
pub struct TotalPerCollectionDb {
    pub total_per_wallet: i64,
    pub name: String,
}

impl From<TotalPerCollectionDb> for TotalPerCollectionData {
    fn from(data: TotalPerCollectionDb) -> Self {
        Self {
            total_per_wallet: data.total_per_wallet,
            name: data.name,
        }
    }
}
