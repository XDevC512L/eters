use actix_web::{ get, HttpResponse, post, Responder};
use actix_web::http::StatusCode;

// this endpoint checks the existence of an EteBase Server
// In the reference implementation it returns null in the http body but
// I suppose clients only check the http status code
#[get("/is_etebase/")]
pub async fn is_etebase_endpoint() -> impl Responder {
    return HttpResponse::new(StatusCode::OK).set_body("null");
}

#[post("/login_challenge/")]
pub async fn login_challenge_endpoint() -> impl Responder {
    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR);
}

#[post("/login/")]
pub async fn login_endpoint() -> impl Responder {
    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR);
}

#[post("/logout/")]
pub async fn logout_endpoint() -> impl Responder {
    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR);
}
#[post("/signup/")]
pub async fn signup_endpoint() -> impl Responder {
    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR);
}

#[post("/change_password/")]
pub async fn change_password_endpoint() -> impl Responder {
    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR);
}

#[post("/dashboard_url/")]
pub async fn dashboard_url_endpoint() -> impl Responder {
    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR);
}