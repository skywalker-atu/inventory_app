use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{NewItem, Item};


//use sqlx::Row;

#[post("/items")]
async fn add_item(pool: web::Data<PgPool>, item: web::Json<NewItem>) -> impl Responder {
    let query = "INSERT INTO items (name, quantity, price) VALUES ($1, $2, $3) RETURNING id";
    
    let result = sqlx::query(query)
        .bind(&item.name)
        .bind(item.quantity)
        .bind(item.price)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().body("Item added successfully"),
        Err(err) => {
            eprintln!("Error adding item: {:?}", err);
            HttpResponse::InternalServerError().body("Error adding item")
        }
    }
}

#[get("/items")]
async fn get_items(pool: web::Data<PgPool>) -> impl Responder {
    let query = "SELECT id, name, quantity, price FROM items";
    
    let result: Result<Vec<Item>, sqlx::Error> = sqlx::query_as::<_, Item>(query)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items), // Return items as JSON on success
        Err(err) => {
            eprintln!("Error fetching items: {:?}", err); // Log the error for debugging
            HttpResponse::InternalServerError().body("Error fetching items") // Send error response
        }
    }
}

#[put("/items/{id}")]
async fn update_item(pool: web::Data<PgPool>, item_id: web::Path<i32>, item: web::Json<NewItem>) -> impl Responder {
    let query = "UPDATE items SET name = $1, quantity = $2, price = $3 WHERE id = $4";
    let result = sqlx::query(query)
        .bind(&item.name)
        .bind(item.quantity)
        .bind(item.price)
        .bind(item_id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Item updated"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating item"),
    }
}

#[delete("/items/{id}")]
async fn delete_item(pool: web::Data<PgPool>, item_id: web::Path<i32>) -> impl Responder {
    let query = "DELETE FROM items WHERE id = $1";
    let result = sqlx::query(query)
        .bind(item_id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Item deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting item"),
    }
}