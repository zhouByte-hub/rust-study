/**
 * SeaORM 是一个为 Rust 语言设计的、异步的、基于 Active Record 模式的 ORM（对象关系映射）框架。它构建在 sqlx 之上，提供了更高层次的抽象，让你可以用面向对象的方式操作数据库，而无需直接写 SQL。
 * 特点：
 *      1、 异步支持：与 tokio、async-std 等运行时无缝集成。
 *      2、类型安全：编译时检查字段和查询。
 *      3、Active Record 模式：每个模型（Model）对应数据库中的一行，可以直接调用方法进行增删改查。
 *      4、关系支持：轻松处理 has_many、has_one、belongs_to 等关系。
 *      5、迁移支持：配合 sea-orm-cli 工具可管理数据库迁移。
 *      6、代码生成：通过 sea-orm-cli 自动生成模型和实体结构体，减少样板代码。
 * 
 * SeaORM 本身不直接连接数据库，而是通过底层驱动（如 sqlx）支持多种数据库。目前支持的数据库包括：
 *      1、MySQL、MariaDB
 *      2、PostgreSQL
 *      3、SQLite
 * 
 * cargo install sea-orm-cli
 * 安装上述工具可以通过命令生成模型：
 *      sea-orm-cli generate model -u mysql://root:123456@localhost:3306/test -o src/database/models
 * 生成的模型文件会包含实体结构体、查询方法、关联关系等。
 * 
 */

pub mod mysql;
pub mod models;
