mod model;
mod mongo;
use mongo::MongoCon;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use mongodb::bson::doc;

struct State {
    mongo_con: MongoCon
}

#[get("/healthcheck")]
async fn check_health(data: web::Data<State>) -> impl Responder {
    let mongo_con = &data.mongo_con;

    return match mongo_con
        .client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("database queried successfully"),
        Err(_) => HttpResponse::InternalServerError().body("database could not be queried"),
    }
}

#[get("/api/uzivatel")]
async fn get_all_users (data: web::Data<State>) -> impl Responder {
    let mongo_con = &data.mongo_con;

    match mongo_con.get_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => return HttpResponse::InternalServerError().body("database could not be queried"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_con = MongoCon::new().await.expect("could not connect to database");

    return HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State { mongo_con: mongo_con.clone() }))
            .service(check_health)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
