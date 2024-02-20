pub mod entities;

use dotenvy::dotenv;
use http::Method;
use std::convert::TryFrom;
use std::env;
use std::sync::Arc;

use axum::extract::*;
use axum::{async_trait, http::StatusCode};
use axum_extra::extract::CookieJar;

use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};

use petstore_api::*;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

struct ServerState {
    pool: Pool,
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
        let mut conn = self.pool.get().await.unwrap();

        let new_pet = entities::PetEntity {
            id: i32::try_from(body.id).unwrap(),
            name: body.name,
            tag: body.tag,
        };

        let res = diesel::insert_into(entities::pet::table)
            .values(new_pet)
            .returning(entities::PetEntity::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(internal_error)
            .unwrap();

        Ok(CreatePetsResponse::Status201_NullResponse)
    }

    async fn list_pets(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::ListPetsQueryParams,
    ) -> Result<ListPetsResponse, String> {
        let mut conn = self.pool.get().await.unwrap();

        let db_res = entities::pet::table
            .select(entities::PetEntity::as_select())
            .load(&mut conn)
            .await
            .map_err(internal_error);

        let res = db_res
            .unwrap()
            .iter()
            .map(|e| models::Pet {
                id: i64::from(e.id),
                name: e.name.clone(),
                tag: e.tag.clone(),
            })
            .collect();
        Ok(ListPetsResponse::Status200_APagedArrayOfPets {
            body: res,
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
        let mut conn = self.pool.get().await.unwrap();

        let id: i32 = path_params.pet_id.parse().expect("Not a valid number");

        let db_res = entities::pet::table
            .find(id)
            .select(entities::PetEntity::as_select())
            .first(&mut conn)
            .await
            .map_err(internal_error)
            .unwrap();

        Ok(
            ShowPetByIdResponse::Status200_ExpectedResponseToAValidRequest(
                petstore_api::models::Pet {
                    id: i64::from(db_res.id),
                    name: db_res.name.to_string(),
                    tag: db_res.tag,
                },
            ),
        )
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    let app = petstore_api::server::new(Arc::new(ServerState { pool }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
