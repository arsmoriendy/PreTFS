#[cfg(test)]
mod test {
    use crate::db_helpers::chain_tagged_inos;
    use sqlx::{migrate, query, QueryBuilder, Sqlite, SqlitePool};
    use tokio::test;

    #[test]
    async fn migrate() {
        let pool = SqlitePool::connect_lazy("sqlite::memory:").unwrap();

        migrate!().run(&pool).await.unwrap();

        query("SELECT ino, size, blocks, atime, mtime, ctime, crtime, kind, perm, nlink, uid, gid, rdev, blksize, flags FROM file_attrs")
                .execute(&pool)
                .await
                .unwrap();

        query("SELECT ino, name FROM file_names")
            .execute(&pool)
            .await
            .unwrap();

        query("SELECT ino, size, blocks, atime, mtime, ctime, crtime, kind, perm, nlink, uid, gid, rdev, blksize, flags, name FROM readdir_rows")
                .execute(&pool)
                .await
                .unwrap();

        query("SELECT tid, name FROM tags")
            .execute(&pool)
            .await
            .unwrap();

        query("SELECT tid, ino FROM associated_tags")
            .execute(&pool)
            .await
            .unwrap();

        query("SELECT dir_ino, cnt_ino FROM dir_contents")
            .execute(&pool)
            .await
            .unwrap();

        query("SELECT ino, content FROM file_contents")
            .execute(&pool)
            .await
            .unwrap();

        pool.close().await;
    }

    #[test]
    async fn chain_tagged_inos_test() {
        let tags = vec![1u64, 2, 3];

        let mut qb = QueryBuilder::<Sqlite>::new("");

        chain_tagged_inos(&mut qb, &tags)
            .map_err(|_| "failed binding tags")
            .unwrap();

        assert_eq!(
            qb.sql(),
            "SELECT ino FROM associated_tags WHERE tid = ? AND ino IN (SELECT ino FROM associated_tags WHERE tid = ? AND ino IN (SELECT ino FROM associated_tags WHERE tid = ?))"
        )
    }
}
