use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

use eosio_client_api::json_rpc::EOSRPC;
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
pub struct GetAbiRequest {
   account: String,

}
pub async fn index2(info: web::Query<GetAbiRequest>) -> impl Responder {
    let host = "https://api.testnet.eos.io";
    let acc = &info.account;
    match EOSRPC::non_blocking(String::from(host)).await {
        Ok(eos) => {
            match eos.get_abi( acc).await {
                Ok(get_abi) =>
                HttpResponse::Ok().body(format!("{:?}",get_abi)),
                Err(_e) => HttpResponse::NotFound().body(acc),
            }
    },
        Err(e) => HttpResponse::InternalServerError().body(String::from(e.description())),
    }
}
