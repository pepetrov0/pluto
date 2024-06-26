package www

import (
	"crypto/md5"
	"embed"
	"fmt"
	"net/http"

	"github.com/sirupsen/logrus"
)

//go:embed static/*
var staticFilesFS embed.FS

const staticFilesPrefix string = "/f/"

var staticFilesRouter http.Handler = http.StripPrefix(staticFilesPrefix, http.FileServerFS(staticFilesFS))

var staticFileHashCache map[string]string = make(map[string]string)

// Returns the hash of a static file
func getStaticFileHash(name string) string {
	log := logrus.WithFields(logrus.Fields{
		"file": name,
	})

	// construct the correct path
	name = fmt.Sprintf("static/%s", name)

	// if hash is already cached - return it
	if hash, ok := staticFileHashCache[name]; ok {
		return hash
	}

	// read the file, in case of an error - compute the hash of the name
	bytes, err := staticFilesFS.ReadFile(name)
	if err != nil {
		bytes = []byte(name)
	}

	// format the hash as hex of maximum length of 10
	log.Traceln("computing hash..")
	hash := fmt.Sprintf("%x", md5.Sum(bytes))
	hash = fmt.Sprintf("%.10s", hash)

	// insert the hash into the cache
	staticFileHashCache[name] = hash
	return hash
}

// Returns the URL to a static file
func getStaticFileUrl(name string) string {
	hash := getStaticFileHash(name)
	return fmt.Sprintf("/f/static/%s?hash=%s", name, hash)
}
