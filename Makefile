default: opensearch

opensearch:
	docker compose up

clean:
	-docker compose down -v

