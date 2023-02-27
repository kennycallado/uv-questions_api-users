CREATE TABLE IF NOT EXISTS roles (
  id SERIAL PRIMARY KEY,
  name VARCHAR(10) NOT NULL
);

ALTER TABLE roles REPLICA IDENTITY FULL;

INSERT INTO roles (id, name) VALUES
  (1, 'admin'),
  (2,'coord'),
  (3,'thera'),
  (4,'user'),
  (5,'robot'),
  (6,'guest') ON CONFLICT DO NOTHING
  ;
  -- update the sequence to the max id
  -- that prevents in case of a conflict to have a duplicate id
  SELECT setval('roles_id_seq', (SELECT MAX(id) FROM roles));

CREATE PUBLICATION roles_pub FOR TABLE roles;
