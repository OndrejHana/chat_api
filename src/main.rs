mod model;
mod mongo;
mod api;

use crate::{ mongo::MongoCon, model::State };
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use mongodb::bson::doc;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_con = MongoCon::new().await.expect("could not connect to database");

    return HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State { mongo_con: mongo_con.clone() }))

            .service(check_health)

            .service(web::scope("/api")
                .service(web::scope("/uzivatel")
                    .service(api::uzivatel::get_all_users)
                    .service(api::uzivatel::create_user)
                )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
