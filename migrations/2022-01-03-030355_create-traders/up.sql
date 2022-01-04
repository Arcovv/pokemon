CREATE TABLE traders (
  "id" bigint GENERATED always AS IDENTITY PRIMARY KEY,
  "user_id" bigint NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  "balance" int NOT NULL DEFAULT 0,
  "created_at" timestamp WITH time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);