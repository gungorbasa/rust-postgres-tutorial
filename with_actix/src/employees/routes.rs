use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

use crate::employees::Employee;

#[get("/employees")]
async fn find_all() -> impl Responder {
    let employee = Employee {
        id: "id".to_string(),
        name: "John".to_string(),
        department: "IT".to_string(),
        salary: 1000,
        age: 30,
    };
    HttpResponse::Ok().json([employee])
}

#[get("/employees/{id}")]
async fn find(id: web::Path<String>) -> impl Responder {
    let employee = Employee {
        id: id.into_inner(),
        name: "John".to_string(),
        department: "IT".to_string(),
        salary: 1000,
        age: 30,
    };
    HttpResponse::Ok().json(employee)
}

#[post("/employees")]
async fn create() -> impl Responder {
    let employee = Employee {
        id: "1".to_string(),
        name: "John".to_string(),
        department: "IT".to_string(),
        salary: 1000,
        age: 30,
    };
    HttpResponse::Ok().json(employee)
}

#[put("/employees/{id}")]
async fn update(id: web::Path<String>) -> impl Responder {
    let employee = Employee {
        id: id.into_inner(),
        name: "John".to_string(),
        department: "IT".to_string(),
        salary: 1000,
        age: 30,
    };
    HttpResponse::Ok().json(employee)
}

#[delete("/employees/{id}")]
async fn delete(id: web::Path<String>) -> impl Responder {
    let employee = Employee {
        id: id.to_string(),
        name: "John".to_string(),
        department: "IT".to_string(),
        salary: 1000,
        age: 30,
    };
    HttpResponse::Ok().json(employee)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(delete);
}
