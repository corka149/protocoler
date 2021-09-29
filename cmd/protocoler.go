package cmd

import (
	"bufio"
	"fmt"
	"github.com/spf13/cobra"
	"os"
	"strconv"
	"strings"
	"time"
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

type protocolEntry struct {
	entryType string
	saidBy    string
	text      string
	timestamp time.Time
}

const (
	infoMarker     = "i"
	decisionMarker = "d"
	taskMarker     = "t"
	removeMarker   = "r"
	quitMarker     = "q"
)

var protocolEntryType = map[string]string{
	infoMarker:     "info",
	decisionMarker: "decision",
	taskMarker:     "task",
}

func printUsage() {
	fmt.Printf("\nEnter: (i) add Info, (d) add Decision, (t) add Task, (r) Remove entry, (entryId) edit entry OR (q) for Quit: ")
}

func protocol(cmd *cobra.Command, args []string) error {
	reader := bufio.NewReader(os.Stdin)
	printUsage()
	input, err := reader.ReadString('\n')
	protocolEntries := make([]protocolEntry, 10)

	input = strings.TrimSpace(input)

	for input != quitMarker && err == nil {

		err = handleNextCmd(input, protocolEntries)

		if err == nil {
			printUsage()
			input, err = reader.ReadString('\n')
			input = strings.TrimSpace(input)
		}
	}

	return err
}

func handleNextCmd(input string, protocolEntries []protocolEntry) error {
	// Append new entry
	markerType, isType := protocolEntryType[input]
	if isType {
		if err := createEntry(protocolEntries, markerType); err != nil {
			return err
		}
	}

	// Remove entry
	if input == removeMarker {
		removeEntry(protocolEntries)
	}

	// Edit entry
	if input != removeMarker && !isType {
		fmt.Println(input)
		possibleIndex, err := strconv.Atoi(input)
		fmt.Println(possibleIndex)
		if err == nil && possibleIndex < len(protocolEntries) {
			editEntry(protocolEntries, possibleIndex)
		}
	}
	return nil
}

func createEntry(entries []protocolEntry, markerType string) error {
	fmt.Printf("Create entry of type '%s'\n", markerType)
	return nil
}

func removeEntry(protocolEntries []protocolEntry) {
	fmt.Println("Delete an entry ...")
}

func editEntry(protocolEntries []protocolEntry, index int) {
	fmt.Printf("Edit entry %d\n", index)
}
