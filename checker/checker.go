package checker

import (
	"database/sql"
	"supervocab/config"
	"supervocab/db"
	"supervocab/util"
	"time"
)

func StartChecker(database *sql.DB, cfg *config.Config) {
	for {
		checkTokens(database)

		users, err := db.GetAllUsers(database)
		if err != nil {
			util.HandleFatalError(err, "Failed to get users")
		}

		for _, user := range users {
			getCards(user.Token)
		}

		time.Sleep(1 * time.Minute)
	}
}
