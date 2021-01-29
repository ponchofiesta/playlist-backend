DROP PROCEDURE IF EXISTS fritzmigrate;

DELIMITER $$
CREATE PROCEDURE fritzmigrate()
BEGIN
  DECLARE not_found INT DEFAULT FALSE;
  DECLARE dup_key INT DEFAULT FALSE;
  DECLARE `new_date` TIMESTAMP;
  DECLARE `new_artist` VARCHAR(255);
  DECLARE `new_title` VARCHAR(255);
  DECLARE `new_cover_url` VARCHAR(255);
  DECLARE `new_cover_width` SMALLINT(6);
  DECLARE `new_cover_height` SMALLINT(6);
  DECLARE `new_asin` VARCHAR(255);
  DECLARE `new_last_cover_check` DATE;
  
  DECLARE cur1 CURSOR FOR 
  	 SELECT TIMESTAMP(p.`date`, p.`time`) AS `date`, t.artist, t.title, t.coverurl, t.coverwidth, t.coverheight, t.`asin`, t.lastcovercheck
		FROM playlist AS p
		LEFT JOIN tracks AS t ON p.tid = t.id;
		
  DECLARE CONTINUE HANDLER FOR NOT FOUND SET not_found = TRUE;
  DECLARE CONTINUE HANDLER FOR 1062 SET dup_key = TRUE;
  DECLARE EXIT HANDLER FOR SQLEXCEPTION 
    BEGIN
      GET DIAGNOSTICS CONDITION 1 @sqlstate = RETURNED_SQLSTATE, @errno = MYSQL_ERRNO, @message = MESSAGE_TEXT;
      ROLLBACK;
      SELECT @sqlstate, @errno, @message;
    END;
    
  OPEN cur1;
  
  START TRANSACTION;
  
  INSERT INTO stations (`key`, `title`)
    VALUES ("fritz", "Radio Fritz");
  SET @station_id = LAST_INSERT_ID();

  read_loop: LOOP
    FETCH cur1 INTO `new_date`, `new_artist`, `new_title`, `new_cover_url`, `new_cover_width`, `new_cover_height`, `new_asin`, `new_last_cover_check`;
    IF not_found THEN
      LEAVE read_loop;
    END IF;
    
    SET @artist_id = 0;
    SET @song_id = 0;
    SET @play_id = 0;
    
    SELECT id INTO @artist_id 
	 	FROM artists AS a
		WHERE a.`name` = `new_artist` 
		LIMIT 1;
    IF not_found THEN
      SET not_found = FALSE;
    	INSERT INTO artists (`name`) 
		 	VALUES (`new_artist`);
    	SET @artist_id = LAST_INSERT_ID();
    END IF;
    
    SELECT id INTO @song_id 
	 	FROM songs AS s
		WHERE s.artist_id = @artist_id AND s.`title` = `new_title` 
		LIMIT 1;
    IF not_found THEN
      SET not_found = FALSE;
      IF `new_last_cover_check` = "0000-00-00"
      THEN
      	SET `new_last_cover_check` = NULL;
      END IF;
      INSERT INTO songs (`artist_id`, `title`, `cover_url`, `cover_width`, `cover_height`, `asin`, `last_cover_check`) 
			VALUES (@artist_id, `new_title`, `new_cover_url`, `new_cover_width`, `new_cover_height`, `new_asin`, `new_last_cover_check`);
    	SET @song_id = LAST_INSERT_ID();
    END IF;
    
    INSERT INTO plays (`song_id`, `date`, `station_id`) 
	   VALUES (@song_id, `new_date`, @station_id);
  END LOOP;
  
  COMMIT;

  CLOSE cur1;
END$$
DELIMITER ;

CALL fritzmigrate();
