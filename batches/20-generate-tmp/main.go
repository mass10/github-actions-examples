package main

import (
	"fmt"
	"os"
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

func main() {

	createNewFile("test.tmp", "Hello, World!")

	fmt.Println("Ok.")
}
