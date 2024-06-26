package www

import (
	"io"
	"net/http"
	"strings"
)

// Constructs a new router to use with the application
func NewRouter() http.Handler {
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
	router = loggerMiddleware(router)
	return router
}
