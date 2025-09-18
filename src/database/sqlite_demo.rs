#[cfg(test)]
mod sqlite_test {
    use crate::database::models::city;
    use sea_orm::ConnectionTrait;
    use sea_orm::Database;
    use sea_orm::DatabaseConnection;
    use sea_orm::DbBackend;
    use sea_orm::EntityTrait;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test1() {
        let path = PathBuf::from("src/database/sqlite.db");
        let db_url = format!("sqlite:{}?mode=rwc", path.to_str().unwrap());

        let db: DatabaseConnection = Database::connect(&db_url).await.unwrap();
        // 查询
        let cities = city::Entity::find().all(&db).await.unwrap();
        for city in cities {
            println!("{:?}", city);
        }
    }

    #[tokio::test]
    async fn create_table() {
        // 创建一张表
        let path = PathBuf::from("src/database/sqlite.db");
        let db_url = format!("sqlite:{}?mode=rwc", path.to_str().unwrap());

        let db: DatabaseConnection = Database::connect(&db_url).await.unwrap();

        // 尝试查询表是否存在
        let table_exists = db
            .execute(sea_orm::Statement::from_string(
                sea_orm::DbBackend::Sqlite,
                "SELECT name FROM sqlite_master WHERE type='table' AND name='city'".to_string(),
            ))
            .await;

        match table_exists {
            Ok(_) => {
                println!("City 表已存在，跳过创建");
            }
            Err(_) => {
                // 使用 SeaORM 的 Schema 创建表
                let schema = sea_orm::Schema::new(DbBackend::Sqlite);

                // 创建 city 表
                let stmt = schema.create_table_from_entity(city::Entity);
                let builder = db.get_database_backend().build(&stmt);

                // 执行创建表的 SQL 语句
                db.execute(builder).await.unwrap();

                println!("City 表创建成功");
            }
        }
    }
}
