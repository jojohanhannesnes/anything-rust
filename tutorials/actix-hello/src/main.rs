use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>,
}
struct AppState {
    app_name: String,
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test").route(web::get().to(|| async { HttpResponse::Ok().body("test") })),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app").route(web::get().to(|| async { HttpResponse::Ok().body("app") })),
    );
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
async fn index_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

#[get("/sharedmut")]
async fn shared_mut(data: web::Data<AppStateWithCounter>) -> String {
    let mut counters = data.counter.lock().unwrap();
    *counters += 1;
    format!("Counter: {counters}")
}

#[get("/testindex")]
async fn index_state_test(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name} 2")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There!")
}

async fn index() -> impl Responder {
    "hi html"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter_data = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(1),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: "Welcome to Actix web".to_string(),
            }))
            .service(index_state)
            .service(index_state_test)
            .service(hello)
            .app_data(counter_data.clone())
            .service(shared_mut)
            .service(web::scope("/apps").route("/index.html", web::get().to(index)))
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/rustlang")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/rustlang")
                    .guard(guard::Host("users.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
            )
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
