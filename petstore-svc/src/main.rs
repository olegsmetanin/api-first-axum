use async_trait::async_trait;
use std::sync::Arc;
use http::Method;
use axum::extract::*;
use axum_extra::extract::CookieJar;

use petstore_svc::*;

struct ServerState {
    // database: sea_orm::DbConn,
}
 
 #[allow(unused_variables)]
 #[async_trait]
 impl petstore_svc::Api for ServerState {
    
    async fn list_pets(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
          query_params: models::ListPetsQueryParams,
        ) -> Result<ListPetsResponse, String> {

            Ok(ListPetsResponse::Status200_APagedArrayOfPets
                {
                    body: Vec::new(),
                    x_next: None
                })
        }
 }

#[tokio::main]
async fn main() {

    let app = petstore_svc::server::new(Arc::new(ServerState {}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
