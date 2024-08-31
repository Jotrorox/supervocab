package checker

import (
	"database/sql"
	"supervocab/db"
	"supervocab/util"
)

func checkTokens(database *sql.DB) {
	users, err := db.GetAllUsers(database)
	if err != nil {
		util.HandleFatalError(err, "Error getting all users while check")
	}

	for _, id := range findDuplicateTokenIDs(users) {
		err = db.DeleteUserByID(database, id)
		if err != nil {
			util.HandleFatalError(err, "Error while deleting duplicates")
		}
		users = removeUserByID(users, id)
	}

	for _, user := range users {
		if !util.ValidateToken(user.Token) {
			err = db.DeleteUserByID(database, user.ID)
			if err != nil {
				util.HandleFatalError(err, "Error while deleting duplicates")
			}
			users = removeUserByID(users, user.ID)
		}
	}
}
