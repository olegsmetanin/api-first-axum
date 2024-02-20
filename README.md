# api-first-axum
API-First with OpenAPI for Axum web framework

## Generate API for https://github.com/OAI/OpenAPI-Specification/blob/main/examples/v3.0/petstore.yaml
```
$ npx openapi-generator-cli generate -i ./petstore.yaml -g rust-axum -t ./templates -o ./petstore-api --additional-properties=packageName=petstore-api
```

## Petstore-svc: simple implementation

- Run example
```
$ cargo run -p petstore-svc
```
- REST API call
```
$ curl localhost:3000/v1/pets
```

## Petstore-db-svc: implementation with db

- Run Postgres in docker
```
docker run \
    --name postgres \
    -p 5432:5432 \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_PASSWORD=postgres \
    -e POSTGRES_DB=postgres \
    -d postgres
```

- Edit .env file to provide DATABASE_URL

- Install postgres libraries  
```
sudo apt-get install libpq-dev
```
- Install postgres cli for migrations 
```
cargo install diesel_cli --no-default-features --features postgres
```
- Run migrations 
```
diesel migration run
```
- Run App
```
$ cargo run -p petstore-db-svc
```
- REST API PUT call
```

$ curl -X POST 'http://localhost:3000/v1/pets' \
  --header 'Content-Type: application/json' \
  --data-raw '{"id": 3, "name": "pet", "tag": "tag"}'

```
- REST API GET call
```
$ curl localhost:3000/v1/pets
```

