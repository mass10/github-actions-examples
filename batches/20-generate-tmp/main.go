package main

import (
	"fmt"
	"io"
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

func copyFile(src string, dest string) {

	r, err := os.Open(src)
	if err != nil {
		panic(err)
	}
	defer r.Close()

	w, err := os.Create(dest)
	if err != nil {
		panic(err)
	}
	defer w.Close()

	n, err := io.Copy(w, r)
	if err != nil {
		panic(err)
	}

	fmt.Println(n, "bytes copied.")
}

func main() {

	copyFile("../../timestamp.tmp", "../../timestamp.tmp.SNAPSHOT")

	// createNewFile("timestamp.tmp.SNAPSHOT", GetCurrentTimestamp())

	fmt.Println("Ok.")
}
