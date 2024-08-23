package config

import (
	"github.com/BurntSushi/toml"
)

type Config struct {
	Server string
}

func LoadConfig(file string) (*Config, error) {
	var config Config
	if _, err := toml.DecodeFile(file, &config); err != nil {
		return nil, err
	}
	return &config, nil
}
