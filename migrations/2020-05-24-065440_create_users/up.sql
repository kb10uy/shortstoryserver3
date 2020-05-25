CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "email" VARCHAR(256) NOT NULL UNIQUE,
    "screen_name" VARCHAR(256) NOT NULL UNIQUE,
    "display_name" VARCHAR(256) NOT NULL,
    "password_hash" VARCHAR(512) NOT NULL,
    "remember_token" VARCHAR(512) NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('users');
