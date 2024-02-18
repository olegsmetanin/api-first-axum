use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::models;

use crate::{Api,
     ListPetsResponse
};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: Api + 'static,
{
    // build our application with a route
    Router::new()
        .route("/v1/pets",
            get(list_pets::<I, A>)
        )
        .with_state(api_impl)
}


#[tracing::instrument(skip_all)]
fn list_pets_validation(
  query_params: models::ListPetsQueryParams,
) -> std::result::Result<(
  models::ListPetsQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}

/// ListPets - GET /v1/pets
#[tracing::instrument(skip_all)]
async fn list_pets<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<models::ListPetsQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    list_pets_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().list_pets(
      method,
      host,
      cookies,
        query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                ListPetsResponse::Status200_APagedArrayOfPets
                                                    {
                                                        body,
                                                        x_next
                                                    }
                                                => {
                                                    if let Some(x_next) = x_next {
                                                    let x_next = match header::IntoHeaderValue(x_next).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling x_next header - {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                                                        }
                                                    };

                                                    
                                                    {
                                                      let mut response_headers = response.headers_mut().unwrap();
                                                      response_headers.insert(
                                                          HeaderName::from_static("x-next"),
                                                          x_next
                                                      );
                                                    }
                                                    }

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                ListPetsResponse::Status0_UnexpectedError
                                                    (body)
                                                => {

                                                  let mut response = response.status(0);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

