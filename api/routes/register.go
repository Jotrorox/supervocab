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

		req, _ := http.NewRequest(
			"GET",
			"https://api.supernotes.app/v1/user/token",
			nil)

		req.Header.Add("Api-Key", token)

		res, _ := http.DefaultClient.Do(req)

		defer func(Body io.ReadCloser) {
            err := Body.Close()
            if err != nil {
                util.HandleFatalError(err, "could not read response body")
            }
        }(res.Body)

		if 200 != res.StatusCode || token == "" {
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
