use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Conditional compilation for sqlx when either test or ssr feature is enabled
#[cfg(any(test, feature = "ssr"))]
use sqlx;
use crate::models::library_system::LibrarySystem;

// Derive sqlx::FromRow trait for Library struct when either ssr or test feature is enabled
// This trait allows Library instances to be created from a database row
#[cfg_attr(any(feature = "ssr", test), derive(sqlx::FromRow))]
// Derive Serialize and Deserialize traits for Library struct
// These traits allow Library instances to be converted to and from JSON
#[derive(Serialize, Deserialize)]
// Library struct represents a library entity in the application
pub struct Library {
    // Unique identifier for the library
    pub id: Uuid,
    // Name of the library
    pub name: String,
    // URL-friendly version of the library name
    pub slug: String,
    // Street address of the library
    pub street_address: String,
    // City where the library is located
    pub city: String,
    // State or province where the library is located
    pub state_or_province: String,
    // Postal code of the library's location
    pub postal_code: String,
    // Current point balance of the library
    pub point_balance: u64,
    // Identifier of the library system to which the library belongs
    pub library_system_id: Uuid,
}

// CreateLibrary struct represents the data required to create a new library
#[derive(Serialize, Deserialize)]
pub struct CreateLibrary {
    // Name of the library
    pub name: String,
    // URL-friendly version of the library name
    pub slug: String,
    // Street address of the library
    pub street_address: String,
    // City where the library is located
    pub city: String,
    // State or province where the library is located
    pub state_or_province: String,
    // Postal code of the library's location
    pub postal_code: String,
}

// Conditional compilation for sqlx when either test or ssr feature is enabled
#[cfg(any(test, feature = "ssr"))]
// Function to create a new library in the database
pub async fn create_library(pool: &sqlx::PgPool, library: CreateLibrary, library_system: LibrarySystem) -> Result<Library, sqlx::Error> {
    // Execute an SQL query to insert a new library into the database
    let result : Library = sqlx::query_as::<_, Library>(
        "insert into library (name, slug, street_address, city, state_or_province, postal_code, point_balance, library_system_id) values ($1, $2, $3, $4, $5, $6, 0, $7) returning id, name, slug, street_address, city, state_or_province, postal_code, point_balance, library_system_id",
    )
        .bind(library.name)
        .bind(library.slug)
        .bind(library.street_address)
        .bind(library.city)
        .bind(library.state_or_province)
        .bind(library.postal_code)
        .bind(library_system.id)
        .fetch_one(pool).await?;
    // Return the newly created library
    Ok(result)
}

// Conditional compilation for sqlx when either test or ssr feature is enabled
#[cfg(any(test, feature = "ssr"))]
// Function to retrieve a library by its unique identifier from the database
pub async fn get_library_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Library, sqlx::Error> {
    // Execute an SQL query to select a library by its unique identifier
    let result = sqlx::query_as::<_, Library>(
        "select * from library where id = $1"
    ).bind(id).fetch_one(pool).await?;
    // Return the library if found
    Ok(result)
}

// Conditional compilation for sqlx when either test or ssr feature is enabled
#[cfg(any(test, feature = "ssr"))]
// Function to retrieve a library by its URL-friendly slug from the database
pub async fn get_library_by_slug(pool: &sqlx::PgPool, slug: String) -> Result<Library, sqlx::Error> {
    // Execute an SQL query to select a library by its URL-friendly slug
    let result = sqlx::query_as::<_, Library>(
        "select * from library where slug = $1"
    ).bind(slug).fetch_one(pool).await?;
    // Return the library if found
    Ok(result)
}

// Conditional compilation for sqlx when either test or ssr feature is enabled
#[cfg(any(test, feature = "ssr"))]
// Function to delete a library from the database
pub async fn delete_library(pool: &sqlx::PgPool, library: Library) -> Result<u64, sqlx::Error> {
    // Execute an SQL query to delete a library by its unique identifier
    let result = sqlx::query("delete from library where id = $1").bind(library.id).execute(pool).await?;
    // Return the number of rows affected by the delete operation
    Ok(result.rows_affected())
}

// Conditional compilation for sqlx when either test or ssr feature is enabled
#[cfg(any(test, feature = "ssr"))]
// Function to update a library in the database
pub async fn update_library(pool: &sqlx::PgPool, id: Uuid, library: Library) -> Result<Library, sqlx::Error> {
    // Execute an SQL query to update a library by its unique identifier
    let result : Library = sqlx::query_as::<_, Library>(
        r#"
        UPDATE library
        SET name = $1, slug = $2, street_address = $3, city = $4, state_or_province = $5, postal_code = $6, point_balance = $7, library_system_id = $8
        WHERE id = $9
        RETURNING id, name, slug, street_address, city, state_or_province, postal_code, point_balance, library_system_id
        "#,
    )
        .bind(library.name)
        .bind(library.slug)
        .bind(library.street_address)
        .bind(library.city)
        .bind(library.state_or_province)
        .bind(library.postal_code)
        .bind(library.point_balance)
        .bind(library.library_system_id)
        .bind(id)
        .fetch_one(pool)
        .await?;

    // Return the updated library
    Ok(result)
}