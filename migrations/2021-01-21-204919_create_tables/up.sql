CREATE TABLE `artists` (
	`id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT,
	`name` VARCHAR(255) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	PRIMARY KEY (`id`) USING BTREE,
	UNIQUE INDEX `artists_name_unique` (`name`) USING BTREE
)
COLLATE='utf8mb4_unicode_ci'
ENGINE=InnoDB
;

CREATE TABLE `songs` (
	`id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT,
	`artist_id` BIGINT(20) UNSIGNED NOT NULL,
	`title` VARCHAR(255) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	`cover_url` VARCHAR(255) NULL DEFAULT NULL COLLATE 'utf8mb4_unicode_ci',
	`cover_width` SMALLINT(6) UNSIGNED NULL DEFAULT NULL,
	`cover_height` SMALLINT(6) UNSIGNED NULL DEFAULT NULL,
	`asin` VARCHAR(255) NULL DEFAULT NULL COLLATE 'utf8mb4_unicode_ci',
	`last_cover_check` DATE NULL DEFAULT NULL,
	PRIMARY KEY (`id`) USING BTREE,
	UNIQUE INDEX `songs_artist_id_title_unique` (`artist_id`, `title`) USING BTREE,
	CONSTRAINT `songs_artist_id_foreign` FOREIGN KEY (`artist_id`) REFERENCES `playlist`.`artists` (`id`) ON UPDATE RESTRICT ON DELETE CASCADE
)
COLLATE='utf8mb4_unicode_ci'
ENGINE=InnoDB
;

CREATE TABLE `stations` (
	`id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT,
	`key` VARCHAR(255) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	`title` VARCHAR(255) NOT NULL COLLATE 'utf8mb4_unicode_ci',
	PRIMARY KEY (`id`) USING BTREE,
	UNIQUE INDEX `stations_key_unique` (`key`) USING BTREE
)
COLLATE='utf8mb4_unicode_ci'
ENGINE=InnoDB
;

CREATE TABLE `plays` (
	`id` BIGINT(20) UNSIGNED NOT NULL AUTO_INCREMENT,
	`song_id` BIGINT(20) UNSIGNED NOT NULL,
	`date` TIMESTAMP NOT NULL,
	`station_id` BIGINT(20) UNSIGNED NOT NULL,
	PRIMARY KEY (`id`) USING BTREE,
	UNIQUE INDEX `plays_song_id_date_unique` (`song_id`, `date`) USING BTREE,
	INDEX `plays_station_id_foreign` (`station_id`) USING BTREE,
	INDEX `plays_date_index` (`date`) USING BTREE,
	CONSTRAINT `plays_song_id_foreign` FOREIGN KEY (`song_id`) REFERENCES `playlist`.`songs` (`id`) ON UPDATE RESTRICT ON DELETE CASCADE,
	CONSTRAINT `plays_station_id_foreign` FOREIGN KEY (`station_id`) REFERENCES `playlist`.`stations` (`id`) ON UPDATE RESTRICT ON DELETE RESTRICT
)
COLLATE='utf8mb4_unicode_ci'
ENGINE=InnoDB
;
