CREATE VIEW IF NOT EXISTS readdir_rows AS
SELECT
  file_attrs.ino,
  file_attrs.size,
  file_attrs.blocks,
  file_attrs.atime,
  file_attrs.mtime,
  file_attrs.ctime,
  file_attrs.crtime,
  file_attrs.kind,
  file_attrs.perm,
  file_attrs.nlink,
  file_attrs.uid,
  file_attrs.gid,
  file_attrs.rdev,
  file_attrs.blksize,
  file_attrs.flags,
  file_names.name
FROM file_attrs
INNER JOIN file_names ON file_attrs.ino = file_names.ino;
