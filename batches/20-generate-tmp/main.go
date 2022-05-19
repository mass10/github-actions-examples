package main

import (
	"fmt"
	"os"
	"time"
)

// テキストファイルを作成します。
func createNewFile(path string, content string) {

	file, err := os.Create(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	file.WriteString(content)
}

func GetCurrentTimestamp() string {

	now := time.Now()
	return now.String()[0:23]
}

func main() {

	createNewFile("test.tmp", GetCurrentTimestamp())

	fmt.Println("Ok.")
}
