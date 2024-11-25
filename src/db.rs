use sqlx::PgPool;
use std::env;
use dotenv::dotenv;

// Function to create a connection pool to the PostgreSQL database
pub async fn create_pool() -> PgPool {
    dotenv().ok(); // Load environment variables from the .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Creating and returning a connection pool
    PgPool::connect(&database_url).await.expect("Failed to create pool")
}
