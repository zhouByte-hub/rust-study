#[cfg(test)]
mod mysql_test {
    use crate::database::models::dict;
    use sea_orm::{
        ActiveModelTrait, ActiveValue, ColumnTrait, Database, EntityTrait, QueryFilter, QueryOrder,
        prelude::Expr,
    };

    #[tokio::test]
    async fn connect_test() {
        let _db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        println!("数据库连接成功");
    }

    #[tokio::test]
    async fn insert_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");

        let obj = dict::ActiveModel {
            id: ActiveValue::NotSet, // 对于自增长的字段可以使用NotSet来标注
            dict_name: ActiveValue::Set("测试字典".to_string()),
            dict_code: ActiveValue::Set("test_dict".to_string()),
            remark: ActiveValue::Set(Some("这是一个测试字典".to_string())),
            group_code: ActiveValue::Set("test_group".to_string()),
        };
        let result = obj.insert(&db).await.expect("插入失败");
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn query_all_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let list = dict::Entity::find()
            .order_by_asc(dict::Column::Id)
            .all(&db)
            .await
            .unwrap();
        for item in list {
            println!("{:?}", item);
        }
    }

    #[tokio::test]
    async fn query_by_id_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let dict = dict::Entity::find_by_id(34).one(&db).await.unwrap();
        match dict {
            Some(item) => println!("{:?}", item),
            None => println!("未查询到数据"),
        }
    }

    #[tokio::test]
    async fn filter_query_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let list = dict::Entity::find()
            .filter(dict::Column::GroupCode.eq("CART_TYPE"))
            .all(&db)
            .await
            .unwrap();
        for item in list {
            println!("{:?}", item);
        }
    }

    #[tokio::test]
    async fn update_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let dict = dict::Entity::find_by_id(34).one(&db).await.unwrap();
        if let Some(item) = dict {
            // 方式一：转换为ActiveModel然后更新
            let mut active_item: dict::ActiveModel = item.into();
            active_item.dict_code = ActiveValue::Set("abc".to_string());
            let result = active_item.update(&db).await.unwrap();
            println!("{:?}", result); // 返回的是更新后的Model
        }
    }

    #[tokio::test]
    async fn update_test_2() {
        // 可以先不用进行查询就直接修改，存在则更新成功，如果不存在则不更新也不报错
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let update = dict::Entity::update_many()
            .col_expr(dict::Column::DictCode, Expr::value("update_test_2"))
            .filter(dict::Column::Id.eq(35));

        let result = update.exec(&db).await.unwrap();
        println!("{:?}", result); // UpdateResult { rows_affected: 1 }
    }

    #[tokio::test]
    async fn delete_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let result = dict::Entity::delete_by_id(34).exec(&db).await.unwrap();
        println!("{:?}", result); // DeleteResult { rows_affected: 1 }
    }

    #[tokio::test]
    async fn filter_delete_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let result = dict::Entity::delete_many()
            .filter(dict::Column::Id.eq(34))
            .exec(&db)
            .await
            .unwrap();
        println!("{:?}", result); // DeleteResult { rows_affected: 1 }
    }
}
