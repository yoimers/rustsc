CREATE TABLE posts (
  userId INTEGER,
  postId INTEGER,
  PRIMARY KEY(userId, postId)
) DEFAULT CHARACTER SET= utf8mb4;