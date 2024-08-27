package checker

import (
    "database/sql"
    "supervocab/config"
    "supervocab/db"
    "supervocab/util"
    "time"
)

func StartChecker(database *sql.DB, cfg *config.Config)  {
	for {
		checkTokens(database)

		time.Sleep(1 * time.Minute)
	}
}

func checkTokens(database *sql.DB) {
	users, err := db.GetAllUsers(database)
	if err != nil {
		util.HandleFatalError(err, "Error getting all users while check")
	}
	
	// Check for and remove duplicates
    for _, id := range findDuplicateTokenIDs(users) {
        err = db.DeleteUserByID(database, id)
		if err != nil {
			util.HandleFatalError( err, "Error while deleting duplicates")
		}
		users = removeUserByID(users, id)
	}
	
	for _, user := range users {
		if !util.ValidateToken(user.Token) {
			err = db.DeleteUserByID(database, user.ID)
			if err != nil {
				util.HandleFatalError( err, "Error while deleting duplicates")
			}
			users = removeUserByID(users, user.ID)
		}
	} 
}


func findDuplicateTokenIDs(users []util.User) []int {
	seenTokens := make(map[string]bool)
	duplicateIDs := make(map[int]bool)

	for _, user := range users {
		if seenTokens[user.Token] {
			duplicateIDs[user.ID] = true
		} else {
			seenTokens[user.Token] = true
		}
	}

	var ids []int
	for id := range duplicateIDs {
		ids = append(ids, id)
	}

	return ids
}

func removeUserByID(users []util.User, idToRemove int) []util.User {
    for i, user := range users {
        if user.ID == idToRemove {
            return append(users[:i], users[i+1:]...)
        }
    }
    return users
}
