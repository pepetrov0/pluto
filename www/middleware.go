package www

import (
	"net/http"
	"strings"
	"time"

	"github.com/sirupsen/logrus"
)

// A logger middleware that logs every request
func loggerMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		next.ServeHTTP(w, r)
		elapsed := time.Since(start)

		log := logrus.WithFields(logrus.Fields{
			"method":  r.Method,
			"path":    r.URL.Path,
			"elapsed": elapsed,
		})
		log.Traceln("processed request.")
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
