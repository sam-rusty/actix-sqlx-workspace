start app:
	cargo watch -x "run --bin <app name>"

lambda_build:
	sudo cargo lambda build --release --arm64 --bin <app name>

lambda_deploy:
	sudo cargo lambda deploy <app name>

recreate_db:
	sqlx database drop -y && sqlx database create && sqlx migrate run

dev_epic:
	cargo watch -x "run --bin epic"

test:
	cargo nextest run --test-threads=1 --retries=1 --all-features --no-fail-fast