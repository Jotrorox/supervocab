package main

import (
	"supervocab/api"
	"supervocab/config"
	"supervocab/db"
)

func main() {
	cfg, err := config.LoadConfig("config.toml")
	if err != nil {
		panic(err)
	}

	database, err := db.Connect(cfg)
	if err != nil {
		panic(err)
	}

	err = db.CreateUsersTable(database)
	if err != nil {
		panic(err)
	}

	defer api.SetupAPI(database).Listen(":3000")
}
