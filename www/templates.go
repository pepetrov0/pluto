package www

import (
	"embed"
	"html/template"
)

//go:embed templates/*/*.html
var templateFilesFS embed.FS

var templates *template.Template = func() *template.Template {
	result, err := template.New("templates").Funcs(template.FuncMap{
		"surl": getStaticFileUrl,
	}).ParseFS(templateFilesFS, "templates/*/*.html")
	return template.Must(result, err)
}()
