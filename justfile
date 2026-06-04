set shell := ["bash", "-c"]

# Extract version from Cargo.toml
version := `grep '^version =' Cargo.toml | cut -d '"' -f 2`

# Build the WASM package and prepare assets
build:
    wasm-pack build --target web
    mkdir -p public
    cp index.html public/
    sed "s/__VERSION__/v{{version}}/g" komment-embed.js > public/komment-embed.js
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
