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
		RunE: protocolCmd,
	}

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

// ===== PRIVATE =====

type lineReaderFn func() (string, error)

type protocolEntry struct {
	entryType string
	saidBy    string
	text      string
	timestamp time.Time
}

func (pe protocolEntry) String() string {
	eType := strings.ToUpper(pe.entryType)
	timestamp := pe.timestamp.Format("2006-01-02 15:04")

	return fmt.Sprintf("%s %s - %s: %s", timestamp, eType, pe.saidBy, pe.text)
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

func protocolCmd(cmd *cobra.Command, args []string) error {
	reader := bufio.NewReader(os.Stdin)

	var lineReader = func() (string, error) {
		input, err := reader.ReadString('\n')

		if err != nil {
			return "", err
		}

		return strings.TrimSpace(input), nil
	}

	protocolEntries := make([]*protocolEntry, 0)

	protocolEntries, err := protocol(lineReader, protocolEntries)

	if err != nil {
		return err
	}

	// New
	fmt.Println("===== ===== ===== ===== =====")

	for _, entry := range protocolEntries {
		if entry != nil {
			fmt.Println(entry)
		}
	}

	return nil
}

func protocol(lineReader lineReaderFn, protocolEntries []*protocolEntry) ([]*protocolEntry, error) {
	printUsage()
	input, err := lineReader()

	for input != quitMarker && err == nil {

		protocolEntries, err = handleNextInput(lineReader, input, protocolEntries)

		if err == nil {
			printUsage()
			input, err = lineReader()
		}
	}

	return protocolEntries, err
}

func handleNextInput(lineReader lineReaderFn, input string, protocolEntries []*protocolEntry) ([]*protocolEntry, error) {
	var err error

	entryType, isType := protocolEntryType[input]

	// Append new entry
	if isType {

		protocolEntries, err = createEntry(lineReader, protocolEntries, entryType)

		if err != nil {
			return nil, err
		}
	}

	// Remove entry
	if input == removeMarker {
		protocolEntries, err = removeEntry(lineReader, protocolEntries)
	}

	// Edit entry
	if input != removeMarker && !isType {
		fmt.Println(input)

		possibleIndex, err := strconv.Atoi(input)

		if err == nil && possibleIndex < len(protocolEntries) {
			protocolEntries, err = editEntry(lineReader, protocolEntries, possibleIndex)
		}
	}

	return protocolEntries, nil
}

func createEntry(lineReader lineReaderFn, protocolEntries []*protocolEntry, entryType string) ([]*protocolEntry, error) {
	fmt.Printf("Create entry of type '%s'\n", entryType)

	fmt.Println("---Said by:")
	saidBy, err := lineReader()

	if err != nil {
		return nil, err
	}

	fmt.Println("---Note:")
	note, err := lineReader()

	if err != nil {
		return nil, err
	}

	now := time.Now()

	newEntry := &protocolEntry{
		entryType: entryType,
		saidBy:    saidBy,
		text:      note,
		timestamp: now,
	}

	protocolEntries = append(protocolEntries, newEntry)
	fmt.Printf("\nAdded entry with ID ~> %d <~\n", len(protocolEntries)-1)

	return protocolEntries, nil
}

func removeEntry(lineReader lineReaderFn, protocolEntries []*protocolEntry) ([]*protocolEntry, error) {
	fmt.Println("Delete an entry by ID:")

	indexStr, err := lineReader()

	if err != nil {
		return protocolEntries, err
	}

	index, err := strconv.Atoi(indexStr)

	if err != nil {
		fmt.Println(err)
		return protocolEntries, nil
	}

	// Set to nil is fast and cheap
	protocolEntries[index] = nil

	return protocolEntries, nil
}

func editEntry(lineReader lineReaderFn, protocolEntries []*protocolEntry, index int) ([]*protocolEntry, error) {
	entry := protocolEntries[index]

	fmt.Printf("---Said by ['%s']:\n", entry.saidBy)
	saidBy, err := lineReader()

	if err != nil {
		return nil, err
	}

	if len(saidBy) > 0 {
		entry.saidBy = saidBy
	}

	fmt.Printf("---Note ['%s']:\n", entry.text)
	note, err := lineReader()

	if err != nil {
		return nil, err
	}

	if len(note) > 0 {
		entry.text = note
	}

	return protocolEntries, nil
}
