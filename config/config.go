package config

import (
	"os"
	"path/filepath"
	"strings"

	"gopkg.in/yaml.v3"
)

type Config struct {
	DonutDir string `yaml:"donut_dir"`
}

func Load() (*Config, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return nil, err
	}

	configPath := filepath.Join(homeDir, ".donut.yml")
	defaultDonutDir := filepath.Join(homeDir, ".donut")

	config := &Config{
		DonutDir: defaultDonutDir,
	}

	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		return config, nil
	}

	data, err := os.ReadFile(configPath)
	if err != nil {
		return config, nil
	}

	if err := yaml.Unmarshal(data, config); err != nil {
		return config, nil
	}

	if config.DonutDir == "" {
		config.DonutDir = defaultDonutDir
	}

	// Handle tilde expansion
	if strings.HasPrefix(config.DonutDir, "~/") {
		config.DonutDir = filepath.Join(homeDir, config.DonutDir[2:])
	} else if !filepath.IsAbs(config.DonutDir) {
		config.DonutDir = filepath.Join(homeDir, config.DonutDir)
	}

	return config, nil
}

func (c *Config) Save() error {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return err
	}

	configPath := filepath.Join(homeDir, ".donut.yml")

	data, err := yaml.Marshal(c)
	if err != nil {
		return err
	}

	return os.WriteFile(configPath, data, 0644)
}