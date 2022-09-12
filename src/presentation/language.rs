use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data, 
    HttpResponse,
    http::{header::ContentType, StatusCode}
};

use serde::{Serialize, Deserialize};
use derive_more::{Display};

#[derive(Serialize, Deserialize)]
pub struct LanguageIdentifier {
    programming_language_identifier: String,
}

#[get("/language/{programming_language_identifier}")]
pub async fn get_language(language_identifier: Path<LanguageIdentifier>) -> Json<String> {
    return Json(language_identifier.into_inner().programming_language_identifier);
}