package main

import (
	"os"
	"os/signal"
	"sync"
	"syscall"

	"cosmossdk.io/core/appconfig"
	"cosmossdk.io/depinject"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

type Config struct {
	AppConfigPath        string
	Viper                *viper.Viper
	ExtraDepinjectConfig depinject.Config
}

func (c Config) Run() error {
	configs := []depinject.Config{}
	if c.ExtraDepinjectConfig != nil {
		configs = append(configs, c.ExtraDepinjectConfig)
	}
	configs = append(configs, serverOnlyModules...)

	appConfigPath := c.AppConfigPath
	if appConfigPath == "" {
		appConfigPath = "app_config.json"
	}

	if _, err := os.Stat(appConfigPath); !os.IsNotExist(err) {
		bz, err := os.ReadFile(appConfigPath)
		if err != nil {
			return err
		}

		appConfig := appconfig.LoadJSON(bz)
		configs = append(configs, depinject.Supply(appConfig))
	}

	var serverModules []Module
	err := depinject.Inject(
		depinject.Configs(configs...),
		&serverModules,
	)
	if err != nil {
		return err
	}

	v := c.Viper
	if v == nil {
		v = viper.GetViper()
		v.SetConfigName("config")
		v.AddConfigPath(".")
		v.SetConfigType("json")
	}
	err = v.ReadInConfig()
	if err != nil {
		panic(err)
	}

	rootCmd := &cobra.Command{
		RunE: func(cmd *cobra.Command, args []string) error {
			return nil
		},
	}

	for _, module := range serverModules {
		if hasConfig, ok := module.(HasConfig); ok {
			sub := v.Sub(hasConfig.ServerConfigSection())
			if sub != nil {
				err := sub.Unmarshal(hasConfig.ServerConfigObject())
				if err != nil {
					panic(err)
				}
			}
		}

		if hasCommands, ok := module.(HasCommands); ok {
			for _, cmd := range hasCommands.ServerCommands() {
				rootCmd.AddCommand(cmd)
			}
		}
	}

	rootCmd.AddCommand(&cobra.Command{
		Use: "start",
		RunE: func(cmd *cobra.Command, args []string) error {
			wg := sync.WaitGroup{}
			ctx, _ := signal.NotifyContext(cmd.Context(), syscall.SIGINT, syscall.SIGTERM)

			for _, module := range serverModules {
				hasStart, ok := module.(HasStart)
				if !ok {
					continue
				}

				wg.Add(1)
				go func() {
					defer wg.Done()
					err := hasStart.Start(ctx)
					if err != nil {
						panic(err)
					}
				}()
			}
			wg.Wait()
			return nil
		},
	})

	return rootCmd.Execute()
}

func Run() {
	err := Config{}.Run()
	if err != nil {
		panic(err)
	}
}
