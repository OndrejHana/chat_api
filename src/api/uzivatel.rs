use crate::model::State;
use actix_web::{ get, post, web, HttpResponse, Responder };

#[get("/")]
pub async fn get_all_users (data: web::Data<State>) -> impl Responder {
    let mongo_con = &data.mongo_con;

    match mongo_con.get_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => return HttpResponse::InternalServerError().body("database could not be queried"),
    }
}

#[post("/{username}")]
pub async fn create_user (data: web::Data<State>, username: web::Path<String> ) -> impl Responder {
    let mongo_con = &data.mongo_con;
    let username = username.into_inner();

    match mongo_con.create_user(username).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}

