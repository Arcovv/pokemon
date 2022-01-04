CREATE TABLE traders_balance_logs (
  "id" bigint GENERATED always AS IDENTITY PRIMARY KEY,
  "previous_value" int NOT NULL,
  "current_value" int NOT NULL,
  "modify_value" int NOT NULL,
  "reason" text NOT NULL,
  "created_at" timestamp WITH time zone NOT NULL DEFAULT CURRENT_TIMESTAMP
);