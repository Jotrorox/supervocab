package routes

import (
    "database/sql"
    "io"
    "log"
    "net/http"
    "supervocab/db"
    "supervocab/util"

    "github.com/gofiber/fiber/v2"
)

func RegisterUserHandler(database *sql.DB) func(c *fiber.Ctx) error {
	return func(c *fiber.Ctx) error {
		token := c.Params("token")

		if !util.ValidateToken(token) || token == "" {
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
	}
}
