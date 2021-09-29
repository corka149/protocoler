package cmd

import (
	"bufio"
	"fmt"
	"github.com/spf13/cobra"
	"os"
)

func Execute() {
	var rootCmd = &cobra.Command{
		Use:   "protocoler",
		Short: "A minimalistic typer for protocols",
		Long: `A Fast and minimalistic protocol generator built powered by Cobra & Go.
                It can output the protocol in different formats.`,
		RunE: protocol,
	}

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}


// ===== PRIVATE =====


var marker = map[string]string {
	"i": "info",
	"d": "decision",
	"t": "task",
	"q": "quit",
}

func protocol(cmd *cobra.Command, args []string) error {
	reader := bufio.NewReader(os.Stdin)
	input, err := reader.ReadString('\n')

	for input != "q" && err != nil {

		input, err = reader.ReadString('\n')
	}

	return err
}
