package checker

import (
	"fmt"
	"io"
	"net/http"
	"strings"
	"supervocab/util"
)

func get_cards(token string) {
	url := "https://api.supernotes.app/v1/cards/get/select"

	payload := strings.NewReader("{\n  \"include_membership_statuses\": [\n    2\n  ],\n  \"filter_group\": {\n    \"operator\": \"and\",\n    \"filters\": [\n      {\n        \"type\": \"tag\",\n        \"operator\": \"contains\",\n        \"arg\": \"supervocab\"\n      }\n    ]\n  },\n  \"sort_type\": 0,\n  \"sort_ascending\": true,\n  \"limit\": 0\n}")

	req, _ := http.NewRequest("POST", url, payload)

	req.Header.Add("Api-Key", token)
	req.Header.Add("Content-Type", "application/json")

	res, _ := http.DefaultClient.Do(req)

	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			util.HandleFatalError(
				err,
				"Failed to close response body")
		}
	}(res.Body)
	body, _ := io.ReadAll(res.Body)

	fmt.Println(res)
	fmt.Println(string(body))
}
