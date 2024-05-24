CREATE TABLE "sessions" (
  "id" INTEGER NOT NULL UNIQUE,
  "user_id" INTEGER NOT NULL,
  "agent" TEXT NOT NULL,
  "created_on" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY("id" AUTOINCREMENT),
  FOREIGN KEY("user_id") REFERENCES "users" ("id") ON UPDATE CASCADE ON DELETE CASCADE
);

