CREATE TABLE IF NOT EXISTS dir_contents (
  dir_ino INTEGER,
  cnt_ino INTEGER,
  FOREIGN KEY (dir_ino) REFERENCES file_attrs(ino)
    ON UPDATE CASCADE
    ON DELETE CASCADE,
  FOREIGN KEY (cnt_ino) REFERENCES file_attrs(ino)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);
