use sqlx::sqlite::SqlitePool;

#[tokio::main()]
async fn main() -> anyhow::Result<()> {
    let pool1 = SqlitePool::connect("sqlite:db1.sqlite").await.unwrap();
    let pool2 = SqlitePool::connect("sqlite:db2.sqlite").await.unwrap();

    let id1 = sqlx::query!("SELECT 1 as a").fetch_all(&pool1).await?;
    let id2 = sqlx::query!("SELECT 2 as b").fetch_all(&pool2).await?;

    println!("{}", id1.first().unwrap().a);
    println!("{}", id2.first().unwrap().b);
    println!("Hello, world!");

    Ok(())
}
