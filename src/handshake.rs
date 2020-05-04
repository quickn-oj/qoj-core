use actix_web::{HttpResponse, get};
use qoj_interface::api::{ApiHeader, Standard, EmptyHeader};
use serde::{Serialize, Deserialize};

const STANDARD: Standard = Standard {
    level: 0,
    version: "1.0",
};

const API_HEADER: ApiHeader = ApiHeader {
    name: "QOJ",
    url: "",
    version: "1.0",
    support_standard: STANDARD,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T: Sized, U: Sized> {
    header: T,
    body: U,
}

#[get("/handshake")]
pub async fn handshake() -> HttpResponse {
    let api_header = API_HEADER;
    let response: Response<EmptyHeader,ApiHeader> = Response {
        header: EmptyHeader {},
        body: api_header,
    };
    HttpResponse::Ok().json(response)
}