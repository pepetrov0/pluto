CREATE TABLE "users" (
  "id" BIGSERIAL NOT NULL UNIQUE,
  "email" TEXT NOT NULL UNIQUE,
  "password" TEXT,
  PRIMARY KEY("id")
);
