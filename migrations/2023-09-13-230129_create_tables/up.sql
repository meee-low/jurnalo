-- Your SQL goes here

CREATE TABLE categories (
	"id"	INTEGER NOT NULL,
	"label"	TEXT NOT NULL UNIQUE,
	"prompt"	TEXT NOT NULL UNIQUE,
	"category_type"	INTEGER NOT NULL DEFAULT 1,
	"disabled_bool" INTEGER NOT NULL DEFAULT 0,
	"extra_info" TEXT,
	PRIMARY KEY("id")
	FOREIGN KEY("category_type")
		REFERENCES "category_types" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION,
	CHECK ("disabled_bool" >= 0 AND "disabled_bool" < 2)
);

CREATE TABLE "entries" (
	"id"	INTEGER NOT NULL,
	"timestamp"	TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"category"	INTEGER,
	"value"	INTEGER,
	"details"	TEXT,
	PRIMARY KEY("id")
	FOREIGN KEY ("category")
		REFERENCES "categories" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION,
	FOREIGN KEY ("value")
		REFERENCES "choices" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION,
	CHECK ("value" IS NOT NULL OR "details" IS NOT NULL)
);

CREATE TABLE "choices" (
	"id"	INTEGER NOT NULL,
	"label"	TEXT NOT NULL,
	"shortcut"	TEXT NOT NULL,
	"disabled_bool" INTEGER NOT NULL DEFAULT 0,
	"category_label" TEXT NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY ("category_label")
		REFERENCES "categories" ("label")
		ON DELETE CASCADE
		ON UPDATE CASCADE,
	CHECK ("disabled_bool" >= 0 AND "disabled_bool" < 2)
);

CREATE TABLE "category_types" (
	"id"	INTEGER NOT NULL,
	"label" TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id")
);

CREATE TABLE "quizzes" (
	"id"	INTEGER NOT NULL,
	"label" TEXT NOT NULL UNIQUE,
	"command" TEXT UNIQUE,
	PRIMARY KEY("id")
);

-- many-to-many
CREATE TABLE "quizzes_to_categories" (
	"id"	INTEGER NOT NULL,
	"quiz_label" TEXT NOT NULL,
	"category_label" TEXT NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY("quiz_label")
		REFERENCES "quizzes" ("label")
		ON DELETE CASCADE
		ON UPDATE CASCADE,
	FOREIGN KEY("category_label")
		REFERENCES "categories" ("label")
		ON DELETE CASCADE
		ON UPDATE CASCADE
);

--- Now prepopulate with some basic configs.

INSERT INTO "category_types" ("id", "label")
	VALUES
		(1, "Multiple Choices"),
		(2, "Free Prompt"),
		(3, "Rating Scale"),
		(4, "Menu Tree"),
		(5, "External Command");
