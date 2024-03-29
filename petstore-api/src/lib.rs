#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use types::*;

pub const BASE_PATH: &str = "/v1";
pub const API_VERSION: &str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreatePetsResponse {
    /// Null response
    Status201_NullResponse,
    /// unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ListPetsResponse {
    /// A paged array of pets
    Status200_APagedArrayOfPets {
        body: Vec<models::Pet>,
        x_next: Option<String>,
    },
    /// unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ShowPetByIdResponse {
    /// Expected response to a valid request
    Status200_ExpectedResponseToAValidRequest(models::Pet),
    /// unexpected error
    Status0_UnexpectedError(models::Error),
}

/// API
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Api {
    /// Create a pet.
    ///
    /// CreatePets - POST /v1/pets
    async fn create_pets(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Pet,
    ) -> Result<CreatePetsResponse, String>;

    /// List all pets.
    ///
    /// ListPets - GET /v1/pets
    async fn list_pets(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::ListPetsQueryParams,
    ) -> Result<ListPetsResponse, String>;

    /// Info for a specific pet.
    ///
    /// ShowPetById - GET /v1/pets/{petId}
    async fn show_pet_by_id(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ShowPetByIdPathParams,
    ) -> Result<ShowPetByIdResponse, String>;
}

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod types;

#[cfg(feature = "server")]
pub(crate) mod header;
