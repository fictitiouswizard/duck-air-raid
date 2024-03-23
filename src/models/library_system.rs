use serde::{Deserialize, Serialize};
use crate::models::library::Library;
use uuid::Uuid;

#[cfg(any(test, feature = "ssr"))]
use sqlx::PgPool;
#[cfg(any(test, feature = "ssr"))]
use sqlx::Error;


#[cfg_attr(any(feature = "ssr", test), derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize)]
pub struct LibrarySystem {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub main_branch_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateLibrarySystem {
    pub name: String,
    pub slug: String,
}

#[cfg(any(test, feature = "ssr"))]
pub async fn create_library_system(pool: &PgPool, ls: CreateLibrarySystem) -> Result<LibrarySystem, Error> {
    let result : LibrarySystem = sqlx::query_as::<_, LibrarySystem>(
        "insert into library_system (name, slug) values ($1, $2) returning id, name, slug, main_branch_id",
    )
        .bind(ls.name)
        .bind(ls.slug)
        .fetch_one(pool).await?;
    Ok(result)
}

#[cfg(any(test, feature = "ssr"))]
pub async fn get_library_system_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<LibrarySystem, Error> {
    let result = sqlx::query_as::<_, LibrarySystem>(
        "select * from library_system where id = $1"
    ).bind(id).fetch_one(pool).await?;
    Ok(result)
}

#[cfg(any(test, feature = "ssr"))]
pub async fn get_library_system_by_slug(pool: &PgPool, slug: String) -> Result<LibrarySystem, Error> {
    let result = sqlx::query_as::<_, LibrarySystem>(
        "select * from library_system where slug = $1"
    ).bind(slug).fetch_one(pool).await?;
    Ok(result)
}

#[cfg(any(test, feature = "ssr"))]
pub async fn delete_library_system(pool: &PgPool, ls: LibrarySystem) -> Result<u64, Error> {
    let result = sqlx::query("delete from library_system where id = $1").bind(ls.id).execute(pool).await?;
    Ok(result.rows_affected())
}

#[cfg(any(test, feature = "ssr"))]
pub async fn update_library_system(pool: &PgPool, id: Uuid, ls: LibrarySystem) -> Result<LibrarySystem, Error> {
    let result : LibrarySystem = sqlx::query_as::<_, LibrarySystem>(
        r#"
        UPDATE library_system
        SET name = $1, slug = $2, main_branch_id = $3
        WHERE id = $4
        RETURNING id, name, slug, main_branch_id
        "#,
    )
        .bind(ls.name)
        .bind(ls.slug)
        .bind(ls.main_branch_id)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(result)
}


#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;
    use super::*;
    use actix_web;
    use actix_web::web::{delete, get};
    use sqlx;
    use sqlx::PgPool;
    // e2c38a9a-72f2-4908-9905-f6901ef9931d

    async fn get_db_connection() -> PgPool {
        let _ = dotenv();
        let db_username = env::var("DB_USERNAME").expect("unable to find DB_USERNAME");
        let db_password = env::var("DB_PASSWORD").expect("unable to find DB_PASSWORD");
        let db_server = env::var("DB_SERVER").expect("unable to find DB_SERVER");
        let db_port = env::var("DB_PORT").expect("unable to find DB_PORT");
        let db_name = env::var("DB_NAME").expect("unable to find DB_NAME");

        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_username,
            db_password,
            db_server,
            db_port,
            db_name,
        );

        return PgPool::connect(&connection_string).await.expect("Unable to connect to db");
    }

    #[actix_web::test]
    async fn test_create_library_system() {
        let connection = get_db_connection().await;

        let library_system = CreateLibrarySystem {
            name: String::from("New Library"),
            slug: String::from("nl"),
        };

        let ls = create_library_system(&connection, library_system).await.expect("Unable to create library");
        assert_eq!(ls.id.is_nil(), false);


        let _ = delete_library_system(&connection, ls).await;
    }

    #[actix_web::test]
    async fn test_get_library_system_by_id() {
        let connection = get_db_connection().await;

        let library_system = CreateLibrarySystem {
            name: String::from("Central Arkansas Library System"),
            slug: String::from("cals"),
        };

        let ls = create_library_system(&connection, library_system).await.expect("failed to create library system");

        let library_system = get_library_system_by_id(&connection, ls.id).await.expect("failed to grab library"); 
        assert_eq!(ls.id, library_system.id);

        let _ = delete_library_system(&connection, ls).await;
    }

    #[actix_web::test]
    async fn test_get_library_system_by_slug() {
        let connection = get_db_connection().await;

        let library_system = CreateLibrarySystem {
            name: String::from("Central Arkansas Library System"),
            slug: String::from("tst1"),
        };

        let library_system = create_library_system(&connection, library_system).await.expect("failed to create library system");
        let retrieved_library_system = get_library_system_by_slug(&connection, String::from("tst1")).await.expect("failed to retreive library system");
        assert_eq!(library_system.slug, retrieved_library_system.slug);

        let _ = delete_library_system(&connection, retrieved_library_system).await;
    }

    #[actix_web::test]
    async fn library_system_update_with_valid_data() {
        let connection = get_db_connection().await;

        let library_system = CreateLibrarySystem {
            name: String::from("Old Library"),
            slug: String::from("ol"),
        };

        let ls = create_library_system(&connection, library_system).await.expect("Unable to create library");

        let updated_library_system = LibrarySystem {
            id: ls.id,
            name: String::from("Updated Library"),
            slug: String::from("ul"),
            main_branch_id: None,
        };

        let updated_ls = update_library_system(&connection, ls.id, updated_library_system).await.expect("Unable to update library");

        assert_eq!(updated_ls.name, "Updated Library");
        assert_eq!(updated_ls.slug, "ul");

        let _ = delete_library_system(&connection, updated_ls).await;
    }

    #[actix_web::test]
    async fn library_system_update_with_invalid_id() {
        let connection = get_db_connection().await;

        let updated_library_system = LibrarySystem {
            id: Uuid::new_v4(),
            name: String::from("Updated Library"),
            slug: String::from("ul"),
            main_branch_id: None,
        };

        let result = update_library_system(&connection, Uuid::new_v4(), updated_library_system).await;

        assert!(result.is_err());

        let error = result.err().unwrap();
        assert_eq!(error.to_string(), "no rows returned by a query that expected to return at least one row");
    }

    #[actix_web::test]
    async fn library_system_update_with_existing_slug() {
        let connection = get_db_connection().await;

        let library_system1 = CreateLibrarySystem {
            name: String::from("Library 1"),
            slug: String::from("l1"),
        };

        let library_system2 = CreateLibrarySystem {
            name: String::from("Library 2"),
            slug: String::from("l2"),
        };

        let ls1 = create_library_system(&connection, library_system1).await.expect("Unable to create library 1");
        let ls2 = create_library_system(&connection, library_system2).await.expect("Unable to create library 2");

        let updated_library_system = LibrarySystem {
            id: ls2.id,
            name: String::from("Updated Library"),
            slug: String::from("l1"),
            main_branch_id: None,
        };

        let result = update_library_system(&connection, ls2.id, updated_library_system).await;

        assert!(result.is_err());

        let error = result.err().unwrap();
        assert_eq!(error.to_string(), "error returned from database: duplicate key value violates unique constraint \"library_system_slug_key\"");

        let _ = delete_library_system(&connection, ls1).await;
        let _ = delete_library_system(&connection, ls2).await;
    }
}
