build:
	@go build -o dist/pluto .

tailwind:
	@tailwind -i styles.css -o www/static/styles.css --minify

default: build