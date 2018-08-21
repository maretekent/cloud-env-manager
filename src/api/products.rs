use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::Future;

use api::index::AppState;
use model::products::{ProductsList};

pub fn products(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!();
}

pub fn products_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(ProductsList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(products_list) => 
                    Ok(HttpResponse::Ok().json(products_list)),
                Err(_) =>
                    Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}