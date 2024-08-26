mod models;
use self::models::*;

use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};

use handlebars::Handlebars;
use serde_json::json;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "CatDex",
        "cats":[
            {
                "name": "Fluffy",
                "image": "static/image/cat001.jpg"
            },
            {
                "name": "Whiskers",
                "image": "static/image/cat002.jpg"
            },
            {
                "name": "Paws",
                "image": "static/image/cat003.jpg"
            }
        ]
    });

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://0.0.0.0:8080");

    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("index", "static/index.html")
        .unwrap();

    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
