package db

import (
	"database/sql"
	"supervocab/config"

	_ "github.com/lib/pq"
)

// Connect establishes a connection to the PostgreSQL database.
//
// It takes a config object containing the server URL as a parameter.
// Returns a sql.DB object and an error if the connection fails.
func Connect(config *config.Config) (*sql.DB, error) {
	db, err := sql.Open("postgres", config.Server)
	if err != nil {
		return db, err
	}
	return db, nil
}

// CreateUsersTable creates the users table in the PostgreSQL database if it does not already exist.
//
// It takes a sql.DB object as a parameter, representing the database connection.
// Returns an error if the table creation fails.
func CreateUsersTable(db *sql.DB) error {
	_, err := db.Query("CREATE TABLE IF NOT EXISTS users(id SERIAL PRIMARY KEY, token TEXT NOT NULL);")
	return err
}

// InsertUser inserts a user into the database.
//
// It takes a sql.DB object and a token string as parameters, representing the database connection and the user's token.
// Returns an error if the insertion fails.
func InsertUser(db *sql.DB, token string) error {
	_, err := db.Query("INSERT INTO users(token) VALUES($1)", token)
	return err
}
