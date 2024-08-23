package main

import (
	"supervocab/api"
	"supervocab/config"
	"supervocab/db"
	"supervocab/util"
)

func main() {
	config, err := config.LoadConfig("config.toml")
	if err != nil {
		util.HandleFatalError(err, "could not load config")
	}

	database, err := db.Connect(config)
	if err != nil {
		util.HandleFatalError(err, "could not connect to database")
	}

	if err := db.CreateUsersTable(database); err != nil {
		util.HandleFatalError(err, "could not create users table")
	}

	defer api.SetupAPI(database).Listen(":3000")
}
