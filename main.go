package main

import (
    "github.com/gofiber/fiber/v2"
    "supervocab/api"
	"supervocab/config"
	"supervocab/db"
	"supervocab/util"
)

func main() {
	cfg, err := config.LoadConfig("config.toml")
	if err != nil {
		util.HandleFatalError(err, "could not load config")
	}

	database, err := db.Connect(cfg)
	if err != nil {
		util.HandleFatalError(err, "could not connect to database")
	}

	if err := db.CreateUsersTable(database); err != nil {
		util.HandleFatalError(err, "could not create users table")
	}

	defer func(api *fiber.App, addr string) {
        err := api.Listen(addr)
        if err != nil {
            util.HandleFatalError(err, "There was an error starting the server")
        }
    }(api.SetupAPI(database), ":3000")
}
