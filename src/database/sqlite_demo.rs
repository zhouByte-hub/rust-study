#[cfg(test)]
mod sqlite_test {
    use crate::database::models::city;
    use crate::database::models::question;
    use sea_orm::ActiveModelTrait;
    use sea_orm::ConnectionTrait;
    use sea_orm::Database;
    use sea_orm::DatabaseConnection;
    use sea_orm::DbBackend;
    use sea_orm::EntityTrait;
    use sea_orm::Schema;
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

    /**
     * 根据实体创建表
     */
    #[tokio::test]
    async fn create_table_v2() {
        let path = PathBuf::from("src/database/sqlite.db");
        let db_url = format!("sqlite:{}?mode=rwc", path.to_str().unwrap());

        let db: DatabaseConnection = Database::connect(&db_url).await.unwrap();

        let backend = db.get_database_backend();
        let schema = Schema::new(backend);
        let stmt: sea_orm::sea_query::TableCreateStatement =
            schema.create_table_from_entity(question::Entity);

        db.execute(backend.build(&stmt)).await.unwrap();
    }

    #[tokio::test]
    async fn insert_data() {
        let path = PathBuf::from("src/database/sqlite.db");
        let db_url = format!("sqlite:{}?mode=rwc", path.to_str().unwrap());

        let db: DatabaseConnection = Database::connect(&db_url).await.unwrap();

        // 创建一个新的 question 记录
        let question = question::ActiveModel {
            id: sea_orm::ActiveValue::NotSet, // 让数据库自动生成 ID
            question: sea_orm::ActiveValue::Set("What is the capital of France?".to_string()),
            options: sea_orm::ActiveValue::Set(serde_json::json!([
                "Paris", "London", "Berlin", "Madrid"
            ])),
            answer: sea_orm::ActiveValue::Set("Paris".to_string()),
            question_category: sea_orm::ActiveValue::Set(1),
            case_category: sea_orm::ActiveValue::Set(1),
            image: sea_orm::ActiveValue::Set(None),
            description: sea_orm::ActiveValue::Set(None),
            create_time: sea_orm::ActiveValue::Set(None),
            update_time: sea_orm::ActiveValue::Set(None),
            create_by: sea_orm::ActiveValue::Set(None),
            update_by: sea_orm::ActiveValue::Set(None),
        };

        // 插入数据
        question.insert(&db).await.unwrap();

        // 查询
        let questions = question::Entity::find().all(&db).await.unwrap();
        for question in questions {
            println!("{:?}", question);
        }
    }

    #[tokio::test]
    async fn update_data() {
        let path = PathBuf::from("src/database/sqlite.db");
        let db_url = format!("sqlite:{}?mode=rwc", path.to_str().unwrap());

        let db: DatabaseConnection = Database::connect(&db_url).await.unwrap();

        let question = question::Entity::find_by_id(2).one(&db).await.unwrap();
        if let Some(question) = question {
            println!("{:?}", question);
            let mut temp: question::ActiveModel = question.into();
            temp.question = sea_orm::ActiveValue::Set("abc".to_string());

            temp.update(&db).await.unwrap();
        }
        let questions = question::Entity::find().all(&db).await.unwrap();
        for question in questions {
            println!("{:?}", question);
        }
    }

    #[tokio::test]
    async fn delete_data() {
        let path = PathBuf::from("src/database/sqlite.db");
        let db_url = format!("sqlite:{}?mode=rwc", path.to_str().unwrap());

        let db: DatabaseConnection = Database::connect(&db_url).await.unwrap();

        let question = question::Entity::find_by_id(2).one(&db).await.unwrap();
        if let Some(question) = question {
            println!("{:?}", question);
            let temp: question::ActiveModel = question.into();
            temp.delete(&db).await.unwrap();
        }
        let questions = question::Entity::find().all(&db).await.unwrap();
        for question in questions {
            println!("{:?}", question);
        }
    }
}
