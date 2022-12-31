docker:
	docker-compose -f ./.dockerfiles/docker-compose.yml up -d --build

run: docker
	RUST_ENV=local cargo run --bin rabbitmq-consumer