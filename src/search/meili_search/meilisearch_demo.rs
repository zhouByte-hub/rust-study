/**
 * meilisearch-sdk = "0.29.1"
 *
 * Meilisearch 是一个强大、快速、开源、易于使用和部署的搜索引擎。
 * API 文档：https://www.meilisearch.com/docs/reference/api/overview
 */

#[cfg(test)]
mod meilisearch_test {
    use crate::search::meili_search::movie::Movie;
    use meilisearch_sdk::settings::Embedder;
    use meilisearch_sdk::settings::EmbedderSource;
    use meilisearch_sdk::{
        client::{Client, SwapIndexes},
        search::{SearchResults, Selectors},
    };
    use std::collections::HashMap;

    /**
     * 新增索引并添加文档
     */
    #[tokio::test]
    async fn insert_test() -> Result<(), Box<dyn std::error::Error>> {
        // 1、创建客户端
        let client = Client::new("http://43.139.97.119:7700", Some("meilisearch-day"))?;
        let file_content = std::fs::read_to_string("src/search/meili_search/movies.json")?;
        let movies: Vec<Movie> = serde_json::from_str(&file_content)?;

        // 2、向索引里添加文档
        let task_result: meilisearch_sdk::task_info::TaskInfo =
            client.index("movies").add_documents(&movies, None).await?;

        // 3、通过任务 ID 查看任务状态
        let task_status = client.get_task(task_result).await?;
        println!("{:?}", task_status);

        Ok(())
    }

    /**
     * 删除索引里所有的文档
     */
    #[tokio::test]
    async fn delete_all_document() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new("http://43.139.97.119:7700", Some("meilisearch-day"))?;

        // 删除所有文档
        let task = client.index("movies").delete_all_documents().await?;

        // 通过任务 ID 查看任务状态
        let task_status = client.get_task(task).await?;
        println!("{:?}", task_status);

        Ok(())
    }

    /**
     * 查询符合条件的文档
     */
    #[tokio::test]
    async fn query_document() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new("http://43.139.97.119:7700", Some("meilisearch-day"))?;

        // 默认情况下，Meilisearch 仅返回搜索查询的前 20 个结果。您可以使用 limit 参数更改此设置。
        let query_result: SearchResults<Movie> = client
            .index("movies_new")
            .search()
            .with_query("Batman")
            .with_limit(1)
            .with_offset(0)
            .with_highlight_post_tag("</mark>")
            .with_highlight_pre_tag("<mark>")
            .with_attributes_to_highlight(Selectors::Some(&["*"]))
            .execute()
            .await?;

        let hits: Vec<meilisearch_sdk::search::SearchResult<Movie>> = query_result.hits;
        println!("查询到的元素个数为：{}", hits.len());
        hits.iter().for_each(|item| {
            println!("{:?}", item);
        });
        Ok(())
    }

    /**
     * 修改文档（ID 必须指定，因为 ID 是文档的唯一标识符）
     */
    #[tokio::test]
    async fn update_document() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new("http://43.139.97.119:7700", Some("meilisearch-day"))?;
        let movie = Movie {
            id: 2,
            title: "Batman".to_string(),
            poster: "https://image.tmdb.org/t/p/w500/ojDg0PGvs6R9xYFodRct2kdI6wC.jpg".to_string(),
            overview: "abcabcabc".to_string(),
            release_date: 0,
            genres: vec!["Action".to_string(), "Adventure".to_string()],
        };
        let task = client
            .index("movies")
            .update_documents_in_batches(&[movie], Some(1), None)
            .await?;
        for item in task {
            println!("============================================");
            let task_status = client.get_task(item).await?;
            println!("{:?}", task_status);
        }
        Ok(())
    }

    /**
     * 设置可显示属性
     */
    #[tokio::test]
    async fn display_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new("http://43.139.97.119:7700", Some("meilisearch-day"))?;
        let displayed_attributes = [
            "id",
            "poster",
            "title",
            "overview",
            "genres",
            "release_date",
        ];
        client
            .index("movies")
            .set_displayed_attributes(&displayed_attributes)
            .await?;
        Ok(())
    }

    /**
     * 字段要么是可搜索的，要么是不可搜索的。
     * 当你进行搜索时，所有可搜索字段都会被检查以匹配查询词，并用于评估文档的相关性，而不可搜索字段则完全被忽略。
     *
     */
    #[tokio::test]
    async fn searchable_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new("http://43.139.97.119:7700", Some("meilisearch-day"))?;
        let searchable_attributes = [
            "id",
            "poster",
            "title",
            "overview",
            "genres",
            "release_date",
        ];
        let task = client
            .index("movies")
            .set_searchable_attributes(&searchable_attributes)
            .await?;
        let task_status = client.get_task(task).await?;
        println!("{:?}", task_status);
        Ok(())
    }

    /**
     * 索引交换
     */
    #[tokio::test]
    async fn swap_index() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new("http://43.139.97.119:7700", Some("meilisearch-day"))?;
        // 1、创建新索引
        client.create_index("movies_new", None).await?;
        let swap_indexes = SwapIndexes {
            indexes: ("movies".to_string(), "movies_new".to_string()),
        };
        // 2、交换索引，原索引中的数据会全部迁移到新索引中
        let task = client.swap_indexes([&swap_indexes]).await?;
        let task_status = client.get_task(task).await?;
        println!("{:?}", task_status);
        Ok(())
    }

    /**
     * 设置 OpenAI
     */
    #[tokio::test]
    async fn embading_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new("http://43.139.97.119:7700", Some("meilisearch-day"))?;
        let embedders = HashMap::from([(
            String::from("test_embedding"),
            Embedder {
                source: EmbedderSource::OpenAi,
                url: Some("https://ai.nengyongai.cn/v1".to_string()),
                api_key: Some("sk-".to_string()),
                model: Some("text-embedding-3-small".to_string()),
                document_template: Some("{{doc.title}}".to_string()),
                dimensions: Some(1536),
                ..Embedder::default()
            },
        )]);
        let embading = client.index("movies").set_embedders(&embedders).await?;
        let task_status = client.get_task(embading).await?;
        println!("{:?}", task_status);
        Ok(())
    }

    /**
     * 列出所有索引
     */
    #[tokio::test]
    async fn indexes_test() -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new("http://43.139.97.119:7700", Some("meilisearch-day"))?;
        let indexes = client.get_indexes().await?;
        for index in indexes.results {
            println!("{:?}", index);
            println!("=========")
        }
        Ok(())
    }
}
