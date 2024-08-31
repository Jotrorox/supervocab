package checker

import (
	"supervocab/util"
)

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
