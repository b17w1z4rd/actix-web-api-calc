use actix_web::{
    HttpServer,
    get,
    App,
    web::Path,
    Responder,
};
use rhai::Engine;
use std::fs;

#[get("/multiply/{num1}/{num2}")]
async fn multiply(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();
    let mut engine = Engine::new();

    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let script = match fs::read_to_string("multiply.rhai") {
        Ok(content) => content,
        Err(_) => return format!("Failed to read multiply.rhai"),
    };

    let result = match engine.eval::<i64>(&script) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error evaluating script: {}", err);
            -1  // Handle the error as per your need
        }
    };

    format!("{}", result)
}

#[get("/add/{num1}/{num2}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();
    let mut engine = Engine::new();

    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let script = match fs::read_to_string("add.rhai") {
        Ok(content) => content,
        Err(_) => return format!("Failed to read add.rhai"),
    };

    let result = match engine.eval::<i64>(&script) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error evaluating script: {}", err);
            -1  // Handle the error as per your need
        }
    };

    format!("{}", result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(multiply)
            .service(add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
