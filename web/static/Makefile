.PHONY: build-css build run

# Build CSS
build-css:
	tailwindcss -i ./src/input.css -o ./public/styles.css --minify

# Docker build
build: build-css
	docker build -t static-site .

# Docker run
run: build
	docker run -p 8080:8080 static-site