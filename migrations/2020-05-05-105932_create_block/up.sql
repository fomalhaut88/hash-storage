CREATE TABLE `block` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `public_key` varchar(128) NOT NULL,
  `data_key` varchar(64) NOT NULL,
  `data_block` text NOT NULL,
  `signature` varchar(128) NOT NULL,
  `secret` varchar(64) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `public_key__key` (`public_key`,`data_key`)
);
