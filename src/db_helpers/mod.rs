pub mod types;

use crate::TagFileSystem;
use fuser::{FileAttr, Request};
use sqlx::{query, query_as, Sqlite};
use std::time::SystemTime;
use types::{from_filetype, from_systime, Bindable, ConvError, DBError, FileAttrRow};

pub fn try_bind_attrs<'q, Q, B>(b: B, a: &FileAttr) -> Result<Q, ConvError>
where
    B: Bindable<'q, Sqlite, Q>,
{
    Ok(
        b.gbind(i64::try_from(a.ino)?) // ino INTEGER
            .gbind(i64::try_from(a.size)?) // size INTEGER,
            .gbind(i64::try_from(a.blocks)?) // blocks INTEGER,
            .gbind(i64::try_from(from_systime(a.atime)?)?) // atime INTEGER,
            .gbind(i64::try_from(from_systime(a.mtime)?)?) // mtime INTEGER,
            .gbind(i64::try_from(from_systime(a.ctime)?)?) // ctime INTEGER,
            .gbind(i64::try_from(from_systime(a.crtime)?)?) // crtime INTEGER,
            .gbind(from_filetype(a.kind)) // kind INTEGER,
            .gbind(a.perm) // perm INTEGER,
            .gbind(a.nlink) // nlink INTEGER,
            .gbind(a.uid) // uid INTEGER,
            .gbind(a.gid) // gid INTEGER,
            .gbind(a.rdev) // rdev INTEGER,
            .gbind(a.blksize) // blksize INTEGER,
            .gbind(a.flags)
            .inner(), // flags INTEGER,
    )
}

impl TagFileSystem<'_> {
    pub async fn ins_attrs(&self, attr: &FileAttr) -> Result<u64, DBError> {
        let q = query_as::<_, (u64,)>( "INSERT INTO file_attrs VALUES (NULL, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15) RETURNING ino");
        Ok(try_bind_attrs(q, attr)?
            .inner()
            .fetch_one(self.pool)
            .await?
            .0)
    }

    pub async fn upd_attrs(&self, attr: &FileAttr) -> Result<(), DBError> {
        let q = query("UPDATE file_attrs SET size = $2, blocks = $3, atime = $4, mtime = $5, ctime = $6, crtime = $7, kind = $8, perm = $9, nlink = $10, uid = $11, gid = $12, rdev = $13, blksize = $14, flags = $15 WHERE ino = $1");
        try_bind_attrs(q, attr)?.execute(self.pool).await?;
        Ok(())
    }

    pub async fn get_ass_tags(&self, ino: u64) -> Result<Vec<u64>, DBError> {
        Ok(
            query_as::<_, (u64,)>("SELECT tid FROM associated_tags WHERE ino = ?")
                .bind(i64::try_from(ino)?)
                .fetch_all(self.pool)
                .await?
                .iter()
                .map(|r| r.0)
                .collect::<Vec<_>>(),
        )
    }

    pub async fn sync_mtime(&self, ino: u64) -> Result<(), DBError> {
        query("UPDATE file_attrs SET mtime = ? WHERE ino = ?")
            .bind(i64::try_from(from_systime(SystemTime::now())?)?)
            .bind(i64::try_from(ino)?)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    pub async fn sync_atime(&self, ino: u64) -> Result<(), DBError> {
        query("UPDATE file_attrs SET atime = ? WHERE ino = ?")
            .bind(i64::try_from(from_systime(SystemTime::now())?)?)
            .bind(i64::try_from(ino)?)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    async fn has_ino_perm(&self, ino: u64, uid: u32, gid: u32, rwx: u16) -> Result<bool, DBError> {
        let p_attrs = query_as::<_, FileAttrRow>("SELECT * FROM file_attrs WHERE ino = ?")
            .bind(i64::try_from(ino)?)
            .fetch_one(self.pool)
            .await?;

        Ok(TagFileSystem::has_perm(
            p_attrs.uid,
            p_attrs.gid,
            p_attrs.perm,
            uid,
            gid,
            rwx,
        ))
    }

    pub async fn req_has_ino_perm(
        &self,
        ino: u64,
        req: &Request<'_>,
        rwx: u16,
    ) -> Result<bool, DBError> {
        Ok(self.has_ino_perm(ino, req.uid(), req.gid(), rwx).await?)
    }
}
