-- Your SQL goes here

CREATE TABLE categories (
	"id"	INTEGER NOT NULL UNIQUE,
	"label"	TEXT NOT NULL UNIQUE,
	"prompt"	TEXT NOT NULL UNIQUE,
	"category_type"	INTEGER NOT NULL DEFAULT 1,
	"disabled_bool" INTEGER NOT NULL DEFAULT 0,
	"extra_info" TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
	FOREIGN KEY("category_type")
		REFERENCES "category_types" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION
);

CREATE TABLE "category_option" (
	"id"	INTEGER NOT NULL UNIQUE,
	"category_id"	INTEGER NOT NULL,
	"option_id"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
	FOREIGN KEY ("category_id")
		REFERENCES "categories" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION,
	FOREIGN KEY ("option_id")
		REFERENCES "options" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION
);

CREATE TABLE "entries" (
	"id"	INTEGER NOT NULL UNIQUE,
	"timestamp"	TIMESTAMP NOT NULL,
	"category"	INTEGER,
	"value"	INTEGER,
	"details"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
	FOREIGN KEY ("category")
		REFERENCES "categories" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION,
	FOREIGN KEY ("value")
		REFERENCES "options" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION
);

CREATE TABLE "options" (
	"id"	INTEGER NOT NULL UNIQUE,
	"label"	TEXT NOT NULL UNIQUE,
	"shortcut"	TEXT NOT NULL UNIQUE,
	"disabled_bool" INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "category_types" (
	"id" INTEGER NOT NULL UNIQUE,
	"label" TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "batteries" (
	"id" INTEGER NOT NULL UNIQUE,
	"label" TEXT NOT NULL UNIQUE,
	"command" TEXT UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "batteries_to_categories" (
	"id" INTEGER NOT NULL UNIQUE,
	"battery_id" INTEGER NOT NULL,
	"category_id" INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
	FOREIGN KEY("battery_id")
		REFERENCES "batteries" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION,
	FOREIGN KEY("category_id")
		REFERENCES "categories" ("id")
		ON DELETE CASCADE
		ON UPDATE NO ACTION
);

--- Now prepopulate with some basic configs.

INSERT INTO "category_types" ("id", "label")
	VALUES
		(1, "Choices"),
		(2, "Free Prompt"),
		(3, "Rating Scale"),
		(4, "Menu Tree"),
		(5, "External Command");

