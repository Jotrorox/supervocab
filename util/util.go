package util

import "log"

// HandleFatalError handles a fatal error by logging it and exiting the program.
//
// err is the error to be logged.
// message is the message to be logged along with the error.
// No return value, as the function calls log.Fatalf.
func HandleFatalError(err error, message string) {
	log.Fatalf("%s: %v", message, err)
}
