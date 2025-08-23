#[cfg(test)]
mod mysql_test {
    use crate::database::models::dict;
    use sea_orm::{
        ActiveModelTrait, ActiveValue, ColumnTrait, Condition, Database, EntityTrait, QueryFilter,
        QueryOrder, QuerySelect, QueryTrait, prelude::Expr,
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
            .order_by_asc(dict::Column::Id) //  排序
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
            // .filter(dict::Column::Id.is_in([1,2,3,4,5]))    // 范围查询
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

    #[tokio::test]
    async fn select_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        /*
           select_only().column(dict::Column::Id)：只输出一个字段，但是可以使用多个column来达到多个字段的目的
           .select_only().columns：输出想要的字段
        */
        let result = dict::Entity::find()
            .select_only()
            .columns([dict::Column::Id, dict::Column::DictCode])
            .all(&db)
            .await;
        match result {
            Ok(ok) => {
                for item in ok {
                    println!("{:?}", item);
                }
            }
            Err(e) => println!("{:?}", e),
        };
    }

    #[tokio::test]
    async fn filter_test_2() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        // select * from dict where id = 34 and dict_code = 'abc'
        let _ = dict::Entity::find()
            .filter(
                Condition::all()
                    .add(dict::Column::Id.eq(34))
                    .add(dict::Column::DictCode.eq("abc")),
            )
            .all(&db)
            .await
            .unwrap();
        // or
        let _ = dict::Entity::find()
            .filter(
                Condition::any()
                    .add(dict::Column::Id.eq(32))
                    .add(dict::Column::DictCode.eq("abc")),
            )
            .all(&db)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn fluent_query() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let id = Some(1);

        // apply_if当id存在时，才会添加filter条件
        let result = dict::Entity::find()
            .apply_if(id, |query, value| query.filter(dict::Column::Id.eq(value)))
            .apply_if(Some(100), QuerySelect::limit)
            .all(&db)
            .await
            .unwrap();

        for item in result {
            println!("{:?}", item);
        }
    }

    #[tokio::test]
    async fn group_by_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        // group by 分组查询
        let result = dict::Entity::find()
            .select_only()
            .column(dict::Column::Id)
            .group_by(dict::Column::Id)
            .all(&db)
            .await
            .unwrap();
        for item in result {
            println!("{:?}", item);
        }
    }
}

/***************************************************高级操作*****************************************************/

#[cfg(test)]
mod advantange_test {
    use crate::database::models::{city, dict, dict_group, driving_school};
    use sea_orm::{
        ActiveModelTrait, ActiveValue, ColumnTrait, Database, EntityTrait, JoinType,
        PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    };

    // 分页
    #[tokio::test]
    async fn pagination_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let mut paginate = dict::Entity::find()
            .order_by_desc(dict::Column::Id)
            .paginate(&db, 10);

        // fetch_page(0); // offset
        // fetch(); // 查询出当前页的数据，使用的是默认值，调用next()可以使默认值 + 1，也就是页数的移动
        // fetch_and_next(); // 在内部一直迭代，知道全部页都走完
        while let Some(item) = paginate.fetch_and_next().await.unwrap() {
            println!("{:?}", item);
        }
        // dict::Entity::find().count(&db).await.unwrap(); // 可以获取总条数
    }

    #[tokio::test]
    async fn many_insert_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let dict_1 = dict::ActiveModel {
            id: ActiveValue::NotSet,
            dict_name: ActiveValue::Set("dict_1".to_string()),
            dict_code: ActiveValue::Set("dict_1".to_string()),
            remark: ActiveValue::Set(Some("dict_1".to_string())),
            group_code: ActiveValue::Set("CART_TYPE".to_string()),
        };
        let dict_2 = dict::ActiveModel {
            id: ActiveValue::NotSet,
            dict_name: ActiveValue::Set("dict_2".to_string()),
            dict_code: ActiveValue::Set("dict_2".to_string()),
            remark: ActiveValue::Set(Some("dict_2".to_string())),
            group_code: ActiveValue::Set("CART_TYPE".to_string()),
        };
        // 正确的批量插入方式：使用Entity::insert_many
        let result = dict::Entity::insert_many(vec![dict_1, dict_2])
            .exec(&db)
            .await
            .unwrap();
        println!("批量插入结果: {:?}", result);
    }

    // save是一个辅助方法，用于将 ActiveModel 保存（插入/更新）到数据库中。
    #[tokio::test]
    async fn save_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");

        let obj = dict::ActiveModel {
            id: ActiveValue::NotSet,
            dict_name: ActiveValue::Set("dict_1".to_string()),
            dict_code: ActiveValue::Set("dict_1".to_string()),
            remark: ActiveValue::Set(Some("dict_1".to_string())),
            group_code: ActiveValue::Set("CART_TYPE".to_string()),
        };

        // 正确的save方法调用：使用实例方法并添加await
        let result = obj.save(&db).await.expect("保存失败");
        println!("保存结果: {:?}", result);
    }

    // 一对多的查询
    #[tokio::test]
    async fn many_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let result = dict_group::Entity::find()
            .filter(dict_group::Column::GroupCode.eq("abc"))
            .find_with_related(dict::Entity)
            .all(&db)
            .await
            .unwrap();
        for item in result {
            println!("dict_group = {:?}", item.0);
            for dict in item.1 {
                println!("dict = {:?}", dict);
            }
        }
        println!("end");
    }

    // 一对一的查询
    #[tokio::test]
    async fn one_to_one_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");

        // 对于一对一来说需要调用 find_also_related 方法
        let result = city::Entity::find()
            .find_also_related(driving_school::Entity)
            .all(&db)
            .await
            .unwrap();
        for model in result {
            println!("city = {:?}", model.0);
            if let Some(school) = model.1 {
                println!("school = {:?}", school);
            }
        }
        println!("end");
    }

    // 要建立join连接的前提是：两个表存在关联关系：一对一、一对多、多对多
    #[tokio::test]
    async fn join_test() {
        let db = Database::connect("mysql://root:123456@127.0.0.1:3306/driver_test")
            .await
            .expect("数据库连接失败");
        let result = dict_group::Entity::find()
            .join(JoinType::LeftJoin, dict_group::Relation::Dict.def())
            .all(&db)
            .await
            .unwrap();
        // 可以在dict_group::Relation::Dict.def().on_condition(|left, right|{})来设置on后面的其他参数，left就是dict_group，right就是dict

        for item in result {
            println!("{:?}", item);
        }
        // left_join、right_join、inner_join、full_join默认使用的是预定义好的关联关系
        // dict_group::Entity::find().left_join(dict::Entity).all(&db).await.unwrap();
    }
}
