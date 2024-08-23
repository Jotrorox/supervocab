package api

import (
	sql "database/sql"
	"supervocab/api/routes"

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

	app.Get("/register/:token", routes.RegisterUserHandler(database))
}
