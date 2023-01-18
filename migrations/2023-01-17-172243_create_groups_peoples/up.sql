CREATE TABLE groups_peoples (
  id INTEGER(11) UNSIGNED PRIMARY KEY AUTO_INCREMENT NOT NULL,
  people_id INTEGER(11) UNSIGNED NOT NULL,
  group_id INTEGER(11) UNSIGNED NOT NULL,
  FOREIGN KEY (people_id) REFERENCES peoples(id),
  FOREIGN KEY (group_id) REFERENCES groups(id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_general_ci';
