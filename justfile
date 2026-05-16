set shell := ["bash", "-c"]

# Build the WASM package and prepare assets
build:
	wasm-pack build --target web
	mkdir -p public/foo
	cp index.html public/
	cp foo.html public/foo/foo.html
	cp komment-embed.js public/
	cp _headers public/
	cp -r pkg public/

# Run the project locally
dev: build
	cd worker && npx wrangler dev

# Deploy to Cloudflare
deploy: build
	cd worker && npx wrangler deploy

# Clean build artifacts
clean:
    rm -rf pkg public target
