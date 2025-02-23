@dev-up:
  docker compose -f compose.dev.yaml up -d
@dev-down:
  docker compose -f compose.dev.yaml stop
  docker compose -f compose.dev.yaml down

# Database
@db-create:
  sqlx database create
@db-migrate *args='':
  sqlx migrate add {{args}}
@db-migrate-run:
  sqlx migrate run
@psql:
  docker exec -it saikyo-db psql -U postgres saikyo_benkyo
