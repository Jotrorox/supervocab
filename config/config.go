package config

import (
	"github.com/BurntSushi/toml"
)

// Config represents the configuration for the application.
//
// Server is the URL of the PostgreSQL server.
type Config struct {
	Server string
}

// LoadConfig loads the configuration from a TOML file.
//
// file is the path to the TOML file containing the configuration.
// Returns a pointer to the loaded Config and an error if the file cannot be decoded.
func LoadConfig(file string) (*Config, error) {
	var config Config
	if _, err := toml.DecodeFile(file, &config); err != nil {
		return nil, err
	}
	return &config, nil
}
