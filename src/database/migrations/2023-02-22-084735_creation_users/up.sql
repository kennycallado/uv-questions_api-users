CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  depends_on INTEGER NOT NULL,
  role_id INTEGER NOT NULL DEFAULT 4,
  user_token VARCHAR(60),
  active BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_users_depends_on FOREIGN KEY (depends_on) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT fk_users_role FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

ALTER TABLE users REPLICA IDENTITY FULL;

SELECT diesel_manage_updated_at('users');
INSERT INTO users (id, user_token, depends_on, role_id) VALUES
  (1, 'admin_user',  1, 1),
  (2, 'coord1_user', 1, 2),
  (3, 'coord2_user', 1, 2),
  (4, 'thera1_user', 2, 3),
  (5, 'thera2_user', 2, 3),
  (6, 'thera3_user', 3, 3),
  (7, 'user1_user',  3, default),
  (8, 'user2_user',  4, default),
  (9, 'user3_user',  5, default),
 (10, 'user4_user',  3, default),
 (11, 'user5_user',  4, 4) ON CONFLICT DO NOTHING
  ;
  -- update the sequence to the max id
  -- that prevents in case of a conflict to have a duplicate id
  SELECT setval('users_id_seq', (SELECT MAX(id) FROM users));

-- puede contener WHERE para filtrar por ativos o con user_token
CREATE PUBLICATION users_pub FOR TABLE users;
