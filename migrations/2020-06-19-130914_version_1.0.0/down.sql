ALTER TABLE `block` DROP KEY `public_group_key__key`;
ALTER TABLE `block` ADD UNIQUE `public_key__key` (`public_key`, `data_key`);

ALTER TABLE `block` DROP COLUMN `data_version`;
ALTER TABLE `block` DROP COLUMN `data_group`;

ALTER TABLE `block` MODIFY COLUMN `data_key` varchar(64) NOT NULL;
