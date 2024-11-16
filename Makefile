default: opensearch

opensearch:
	docker compose up

deploy:
	docker compose up -d

check:
	RUSTFLAGS=-Wunused-crate-dependencies cargo check

clean:
	-docker compose down -v

