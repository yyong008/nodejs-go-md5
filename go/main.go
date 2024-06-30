package main

import (
	"crypto/md5"
	"fmt"
	"io"
	"os"
	"time"
)

func main() {
	filePath := "../video.mkv" // 替换为你的文件路径
	start := time.Now()
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		return
	}
	defer file.Close()

	hash := md5.New()
	if _, err := io.Copy(hash, file); err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}

	md5Hash := fmt.Sprintf("%x", hash.Sum(nil))
	duration := time.Since(start)
	fmt.Printf("MD5 hash of file \"%s\" is: %s\n", filePath, md5Hash)
	fmt.Printf("MD5 file time: \"%s\"", duration)
}
