set dotenv-required
set dotenv-filename := ".env"

bin:
  cargo run

geni-create:
  geni create

geni-up-local:
    geni up

geni-down-local:
    geni down