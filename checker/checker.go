package checker

import (
	"database/sql"
	"supervocab/config"
	"time"
)

func StartChecker(database *sql.DB, cfg *config.Config) {
	for {
		checkTokens(database)

		time.Sleep(1 * time.Minute)
	}
}
