# api-first-axum
API-First with OpenAPI for Axum web framework


```
$ npx openapi-generator-cli generate -i ./petstore.yaml -g rust-axum -t ./templates -o ./petstore-svc --additional-properties=packageName=petstore-svc
$ cargo run -p petstore-svc
```