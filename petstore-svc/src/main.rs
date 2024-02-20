use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::CookieJar;
use http::Method;
use std::sync::Arc;

use petstore_api::*;

struct ServerState {
    // database: sea_orm::DbConn,
}

#[allow(unused_variables)]
#[async_trait]
impl petstore_api::Api for ServerState {
    async fn create_pets(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Pet,
    ) -> Result<CreatePetsResponse, String> {
        Ok(CreatePetsResponse::Status201_NullResponse)
    }

    async fn list_pets(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::ListPetsQueryParams,
    ) -> Result<ListPetsResponse, String> {
        Ok(ListPetsResponse::Status200_APagedArrayOfPets {
            body: Vec::new(),
            x_next: None,
        })
    }

    async fn show_pet_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ShowPetByIdPathParams,
    ) -> Result<ShowPetByIdResponse, String> {
        Ok(
            ShowPetByIdResponse::Status200_ExpectedResponseToAValidRequest(
                petstore_api::models::Pet {
                    id: 1,
                    name: "pet".to_string(),
                    tag: None,
                },
            ),
        )
    }
}

#[tokio::main]
async fn main() {
    let app = petstore_api::server::new(Arc::new(ServerState {}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
