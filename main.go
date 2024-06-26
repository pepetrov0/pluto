package main

import (
	"net/http"

	"github.com/pepetrov0/pluto/www"
	"github.com/sirupsen/logrus"
)

func main() {
	log := logrus.New()
	log.SetLevel(logrus.DebugLevel)

	log.Infoln("listening on :8000")
	http.ListenAndServe(":8000", www.NewRouter(log))
}
