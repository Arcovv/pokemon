# Pokemon

The sample Pokemon trading platform in Rust.

**WARNING: This is not a PRODUCTION project. Just a sample.**

## Get start

### Install diesel and run migrations
```shell
cargo install diesel_cli --no-default-features --features postgres

diesel migration run && cargo fmt
```

### Create and run the postgresql database in docker
```shell
docker run --name Pokemon \
> -e POSTGRES_PASSWORD=password \
> -e POSTGRES_DB=pokemon \
> -p 5436:5432
> -d postgres:12
```

### Create your `.env` file basic from `.env.exaple`
```env
RUST_LOG=actix_web=debug,actix_server=info,debug
HOST=127.0.0.1
CONNECTION_COUNT=5
DATABASE_URL=postgres://postgres:password@127.0.0.1/pokemon
```

### Run the API server
```shell
cargo run -p pokemon_api
```

## Architechture
- All DTO and domain models live in `pokemon_model` module
- All repositories and service live in `pokemon_service` module
- Mock repositories live in `pokemon_test` module
- Third party service live in `pokemon_infrastureture` module
- The HTTP backend is in `pokemon_api`

## API collection

### Show trader latest 50 orders
GET `/api/traders/{trader_id}/orders?limit=50`

### Show card latest 50 orders
GET `/api/cards/{card_id}/orders?limit=50`

### Send a buy order request
POST `/api/traders/{trader_id}/orders/buy`

### Send a sell order request
POST `/api/traders/{trader_id}/orders/sell`

### Run a card matchmaking
POST `/api/matchmaking/cards/{card_id}`
> This API should be replaced by a cronjob, but for now.
>
> 😆😆😆

## How to run in docker
```shell
❯ docker run \
> -e RUST_LOG=actix_web=debug,actix_server=info,debug \
> -e HOST=0.0.0.0 \
> -e CONNECTION_COUNT=5 \
> -e DATABASE_URL=postgres://postgres:password@host.docker.internal:5436/pokemon \
> -p 8666:8666 \
> pokemon_api
```