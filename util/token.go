package util

import (
    "io"
    "net/http"
)

func ValidateToken(token string) bool {
    req, _ := http.NewRequest(
        "GET",
        "https://api.supernotes.app/v1/user/token",
        nil)

    req.Header.Add("Api-Key", token)

    res, _ := http.DefaultClient.Do(req)

    defer func(Body io.ReadCloser) {
        err := Body.Close()
        if err != nil {
            HandleFatalError(err, "could not read response body")
        }
    }(res.Body)

	return res.StatusCode == 200
}
