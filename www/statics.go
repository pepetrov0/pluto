package www

import (
	"crypto/md5"
	"embed"
	"fmt"
	"net/http"
)

//go:embed static/*
var staticFilesFS embed.FS

const staticFilesPrefix string = "/f/"

var staticFilesRouter http.Handler = http.StripPrefix(staticFilesPrefix, http.FileServerFS(staticFilesFS))

func getStaticFileHash(name string) string {
	name = fmt.Sprintf("static/%s", name)
	bytes, err := staticFilesFS.ReadFile(name)
	if err != nil {
		return fmt.Sprintf("%x", md5.Sum([]byte(name)))
	}

	hash := fmt.Sprintf("%x", md5.Sum(bytes))
	return fmt.Sprintf("%.10s", hash)
}

func getStaticFileUrl(name string) string {
	hash := getStaticFileHash(name)
	return fmt.Sprintf("/f/static/%s?hash=%s", name, hash)
}
