https://illuvi-analytics.com

# IlluviAnalytics

Explore comprehensive analytics and insights for Illuvium, the revolutionary crypto gaming project. Track trends, token stats, and performance data all in one place.

## APIs

### ImmutableX

[ImmutableX API](https://docs.x.immutable.com/docs) is used to fetches all the relevant Illuvium Land data for further
analytics usage.
ImmutableX API has a rate limit of 5RPC, see [api-rate-limiting](https://docs.x.immutable.com/docs/api-rate-limiting)

### CoinGecko

[CoinGecko API V3](https://www.coingecko.com/en/api/documentation) is used to query current and historical crypto
prices.
CoinGecko free API has a rate limit of 10-30 calls/minute

### Etherscan
[Etherscan API](https://docs.etherscan.io/api-endpoints/accounts) is used to query initial Land mint data
Etherscan Free plan has a rate limit of 5RPC and 100 000 request per day

## Env

The `dotenvy` crate is used to load environment variables needed for DB connection from `.env` file
Also, feature flags are stored there as well.

### Setting up the environment

Execute [start-local-environment.sh](environment/start-local-environment.sh) to spin up Postgres.
It will execute required DB migrations.

### Site UI
Run `trunk serve`to start the application
