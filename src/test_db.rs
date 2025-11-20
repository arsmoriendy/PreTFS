#[cfg(test)]
mod test {
    use crate::db_helpers::chain_tagged_inos;
    use sqlx::{QueryBuilder, Sqlite};
    use tokio::test;

    #[test]
    async fn chain_tagged_inos_test() {
        let tags = vec![1u64, 2, 3];

        let mut qb = QueryBuilder::<Sqlite>::new("");

        chain_tagged_inos(&mut qb, &tags)
            .map_err(|_| "failed binding tags")
            .unwrap();

        assert_eq!(
            qb.sql(),
            "SELECT ino FROM associated_tags WHERE tid = ? AND ino IN (SELECT ino FROM \
             associated_tags WHERE tid = ? AND ino IN (SELECT ino FROM associated_tags WHERE tid \
             = ?))"
        )
    }
}
