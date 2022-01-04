CREATE TABLE users (
  "id" bigint GENERATED always AS IDENTITY PRIMARY KEY,
  "nickname" text NOT NULL,
  "email" text NOT NULL,
  "password" text NOT NULL,
  "created_at" timestamp WITH time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamp WITH time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);