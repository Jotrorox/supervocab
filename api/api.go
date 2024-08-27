package api

import (
	sql "database/sql"
	"supervocab/api/routes"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
)

// SetupAPI sets up a new Fiber API instance with the provided database connection.
//
// It takes a sql.DB object as a parameter, representing the database connection.
// Returns a *fiber.App object.
func SetupAPI(database *sql.DB) *fiber.App {
	app := fiber.New()

	SetupRoutes(app, database)

	return app
}

// SetupRoutes sets up routes for the fiber app.
//
// It takes a fiber app and a sql database connection as parameters.
// No return value.
func SetupRoutes(app *fiber.App, database *sql.DB) {
	app.Use(cors.New())

	app.Static("/", "./public")

	app.Get("/register/:token", routes.RegisterUserHandler(database))
}
