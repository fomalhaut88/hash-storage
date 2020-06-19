ALTER TABLE `block` ADD COLUMN `data_group` varchar(256) NOT NULL DEFAULT '' AFTER `public_key`;
ALTER TABLE `block` ADD COLUMN `data_version` varchar(32) NOT NULL DEFAULT '' AFTER `data_block`;

ALTER TABLE `block` MODIFY COLUMN `data_key` varchar(256) NOT NULL;
ALTER TABLE `block` MODIFY COLUMN `data_group` varchar(256) NOT NULL;
ALTER TABLE `block` MODIFY COLUMN `data_version` varchar(32) NOT NULL;

ALTER TABLE `block` DROP KEY `public_key__key`;
ALTER TABLE `block` ADD UNIQUE `public_group_key__key` (`public_key`, `data_group`, `data_key`);
