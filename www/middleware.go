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

		fields := logrus.Fields{
			"method":  r.Method,
			"path":    r.URL.Path,
			"elapsed": elapsed,
		}

		if len(r.URL.RawQuery) > 0 {
			fields["query"] = r.URL.RawQuery
		}

		log := logrus.WithFields(fields)
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

// A middleware that redirects on trailing slashes
func redirectOnTrailingSlashMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// redirect on trailing  slash
		path := r.URL.Path
		if strings.HasSuffix(path, "/") && len(path) > 1 {
			http.Redirect(w, r, strings.TrimSuffix(path, "/"), http.StatusMovedPermanently)
			return
		}

		// otherwise continue that chain
		next.ServeHTTP(w, r)
	})
}
