source ./.env
sqlx migrate run
export DATABASE_URL=$DATABASE_URL_TEST
sqlx migrate run