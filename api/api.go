package api

import (
	sql "database/sql"
	"log"
	"supervocab/db"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
)

func SetupAPI(database *sql.DB) *fiber.App {
	app := fiber.New()

	SetupRoutes(app, database)

	return app
}

func SetupRoutes(app *fiber.App, database *sql.DB) {
	app.Use(cors.New())

	app.Get("/register/:token", func(c *fiber.Ctx) error {
		token := c.Params("token")

		if token == "" {
			return c.Status(400).JSON(fiber.Map{
				"status":  "fail",
				"message": "Invalid token",
			})
		}

		if err := db.InsertUser(database, token); err != nil {
			log.Println(err)
			return c.Status(500).JSON(fiber.Map{
				"status":  "fail",
				"message": "Internal Server Error",
			})
		}

		return c.Status(200).JSON(fiber.Map{
			"status":  "success",
			"message": "Registered successfully",
		})
	})
}
