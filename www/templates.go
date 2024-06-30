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
		"navigation_item": func(url string, icon string, text string) map[string]string {
			return map[string]string{
				"Url":  url,
				"Icon": icon,
				"Text": text,
			}
		},
	}).ParseFS(templateFilesFS, "templates/*/*.html")
	return template.Must(result, err)
}()
