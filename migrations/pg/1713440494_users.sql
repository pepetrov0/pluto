CREATE TABLE "users" (
  "id" SERIAL NOT NULL UNIQUE,
  "email" TEXT NOT NULL UNIQUE,
  "password" TEXT,
  PRIMARY KEY("id")
);
