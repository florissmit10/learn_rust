mod todo;

use std::str::FromStr;
use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, error};
use crate::todo::{TodoItem, TodoList};

struct TodoAppState {
    todo_list: Mutex<TodoList>, // <- Mutex is necessary to mutate safely across threads
}


#[get("/")]
async fn index(app_state: web::Data<TodoAppState>) -> Result<impl Responder>{
    let todo_list = app_state.todo_list.lock().unwrap().clone();
    Ok(web::Json(todo_list))

}

#[post("/add")]
async fn add(app_state: web::Data<TodoAppState>, req_body: String) -> Result<impl Responder>{
    let todo_item =
        TodoItem::from_str(&req_body).map_err(error::ErrorBadRequest)?;
    let mut todo_list = app_state.todo_list.lock().unwrap();
    todo_list.add(todo_item);
    Ok(HttpResponse::Ok())
}

#[post("/toggle")]
async fn toggle(app_state: web::Data<TodoAppState>, req_body: String) -> Result<impl Responder> {
    let todo_number = req_body.parse::<u64>().map_err(error::ErrorBadRequest)?;
    let mut todo_list = app_state.todo_list.lock().unwrap();
    todo_list.toggle(todo_number as usize)?;
    Ok(HttpResponse::Ok().body(todo_number.to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(
        TodoAppState {
            todo_list: Mutex::new(TodoList::default())
        });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(add)
            .service(toggle)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}