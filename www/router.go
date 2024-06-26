package www

import (
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"

	"github.com/sirupsen/logrus"
)

// Constructs a new router to use with the application
func NewRouter(log *logrus.Logger) http.Handler {
	fmt.Printf("url of styles.css: %s\n", getStaticFileUrl("styles.css"))

	// create a new global router
	global := http.NewServeMux()

	// handle /health endpoint
	global.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if strings.HasPrefix(r.URL.Path, staticFilesPrefix) {
			staticFilesRouter.ServeHTTP(w, r)
		} else {
			io.WriteString(w, "todo!")
		}
	})
	global.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) { w.WriteHeader(200) })

	// apply middleware
	var router http.Handler = global
	router = cacheControlMiddleware(router)
	router = loggerMiddleware(log, router)
	return router
}

// A logger middleware that logs every request
func loggerMiddleware(log *logrus.Logger, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		next.ServeHTTP(w, r)
		elapsed := time.Since(start)
		log.Debugf("%s %s [%s]\n", r.Method, r.URL.Path, elapsed)
	})
}

// A cache control middleware that adds a cache control header to all responses
func cacheControlMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Cache-Control", "no-store")

		// set immutable cache on static files
		if strings.HasPrefix(r.URL.Path, staticFilesPrefix) {
			w.Header().Set("Cache-Control", "max-age=31536000, immutable")
		}

		next.ServeHTTP(w, r)
	})
}
