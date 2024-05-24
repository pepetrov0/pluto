CREATE TABLE "sessions" (
  "id" BIGSERIAL NOT NULL UNIQUE,
  "user_id" BIGINT NOT NULL REFERENCES "users" ("id") ON UPDATE CASCADE ON DELETE CASCADE,
  "agent" TEXT NOT NULL,
  "created_on" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY("id")
);
