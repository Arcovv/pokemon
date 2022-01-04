CREATE TABLE cards (
  "id" bigint GENERATED always AS IDENTITY PRIMARY KEY,
  "name" text NOT NULL,
  "created_at" timestamp WITH time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);