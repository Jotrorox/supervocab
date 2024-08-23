package db

import (
	"database/sql"
	"supervocab/config"

	_ "github.com/lib/pq"
)

func Connect(config *config.Config) (*sql.DB, error) {
	db, err := sql.Open("postgres", config.Server)
	if err != nil {
		return db, err
	}
	return db, nil
}

func CreateUsersTable(db *sql.DB) error {
	_, err := db.Query("CREATE TABLE IF NOT EXISTS users(id SERIAL PRIMARY KEY, token TEXT NOT NULL);")
	return err
}

func InsertUser(db *sql.DB, token string) error {
	_, err := db.Query("INSERT INTO users(token) VALUES($1)", token)
	return err
}
