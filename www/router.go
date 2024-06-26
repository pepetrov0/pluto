package www

import (
	"net/http"
)

// Constructs a new router to use with the application
func NewRouter() http.Handler {
	// create a new global router
	global := http.NewServeMux()

	// handle /health endpoint
	global.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) { w.WriteHeader(200) })

	// handle static files
	global.Handle(staticFilesPrefix, staticFilesRouter)

	// handle home and fallback
	global.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/" {
			// handle home
			if err := templates.ExecuteTemplate(w, "page.html", nil); err != nil {
				http.Error(w, "", 500)
			}
		} else {
			// handle fallback
			http.Error(w, "fallback", 404)
		}
	})

	// apply middleware
	var router http.Handler = global
	router = cacheControlMiddleware(router)
	router = redirectOnTrailingSlashMiddleware(router)
	router = loggerMiddleware(router)
	return router
}
