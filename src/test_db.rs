#[cfg(test)]
mod test {
    use sqlx::{migrate, query, SqlitePool};

    use crate::*;

    #[test]
    fn migrate() {
        let pool = Box::new(SqlitePool::connect_lazy("sqlite::memory:").unwrap());

        task::block_on(async {
            migrate!().run(pool.as_ref()).await.unwrap();

            query("SELECT * FROM file_attrs")
                .execute(pool.as_ref())
                .await
                .unwrap();

            pool.close().await;
        });
    }
}
