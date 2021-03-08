start:
	cargo run

tests:
	cargo test

.PHONY: docker-up
docker-up:
	docker-compose up --build -d

docker-down:
	docker-compose down

.PHONY: docker-logs
docker-logs:
	docker-compose logs -f keystore-api

.PHONY: docker-exec
docker-exec:
	docker-compose exec keystore-api sh
