CREATE TABLE `block` (
  `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  `public_key` VARCHAR(128) NOT NULL,
  `data_group` VARCHAR(256) NOT NULL,
  `data_key` VARCHAR(256) NOT NULL,
  `data_block` TEXT NOT NULL,
  `data_version` VARCHAR(32) NOT NULL,
  `signature` VARCHAR(128) NOT NULL,
  `secret` VARCHAR(64) NOT NULL,
  UNIQUE(`public_key`, `data_group`, `data_key`)
);
