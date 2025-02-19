CREATE VIEW IF NOT EXISTS readdir_rows AS
SELECT * FROM file_attrs
INNER JOIN file_names ON file_attrs.ino = file_names.ino;
