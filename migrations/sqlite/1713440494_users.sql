CREATE TABLE "users" (
  "id" INTEGER NOT NULL UNIQUE,
  "email" TEXT NOT NULL UNIQUE,
  "password" TEXT,
  PRIMARY KEY("id" AUTOINCREMENT)
);