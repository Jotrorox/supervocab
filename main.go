package main

import (
	"log"

	"database/sql"

	"github.com/BurntSushi/toml"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	_ "github.com/lib/pq"
)

type Config struct {
	Server string
}

func main() {
	var config Config

	if _, err := toml.DecodeFile("config.toml", &config); err != nil {
		panic(err)
	}

	db, err := sql.Open("postgres", config.Server)
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	db.Query("CREATE TABLE IF NOT EXISTS users(id SERIAL PRIMARY KEY, token TEXT NOT NULL);")

	app := fiber.New()
	app.Use(cors.New())

	app.Get("/register/:token", func(c *fiber.Ctx) error {
		token := c.Params("token")

		if token == "" {
			return c.Status(400).JSON(fiber.Map{
				"status":  "fail",
				"message": "Invalid token",
			})
		}

		db.Query("INSERT INTO users(token) VALUES($1)", token)

		return c.Status(200).JSON(fiber.Map{
			"status":  "success",
			"message": "Registered successfully",
		})
	})

	app.Listen(":3000")
}
