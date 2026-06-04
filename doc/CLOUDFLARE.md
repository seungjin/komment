# Deploying to Cloudflare Workers

Komment uses the modern Cloudflare Workers + Static Assets architecture, all implemented in Rust.

## Prerequisites

Ensure you have the following installed:
- `rust`: [Rust](https://www.rust-lang.org/tools/install) (2024 edition)
- `wasm-pack`: `cargo install wasm-pack` (v0.15.0+)
- `worker-build`: `cargo install worker-build`
- `wrangler`: `npm install -g wrangler`
- `just`: `cargo install just` (Optional, for running build commands)

## Unified Configuration

The project is managed through `worker/wrangler.toml`. This file handles:
- **Routing**: Points `main` to the built Rust binary (`build/index.js`).
- **Assets**: Maps `directory = "../public"` to serve your frontend files.
- **Build**: Defines the custom command `worker-build --release` to compile the Rust worker logic.

## Deployment Steps

### Option A: Using Just (Recommended)
From the project root, simply run:
```bash
just deploy
```
This command builds the WASM package, syncs assets to `public/`, and deploys the worker.

### Option B: Manual Deployment
1. **Build the Frontend WASM**:
   ```bash
   wasm-pack build --target web
   ```

2. **Sync Public Assets**:
   The `komment-embed.js` file in the root contains a `__VERSION__` placeholder that must be replaced with the version from `Cargo.toml` during the build process.

   ```bash
   # Extract version from Cargo.toml
   VERSION=$(grep '^version =' Cargo.toml | cut -d '"' -f 2)

   # Create public directory
   mkdir -p public

   # Copy static files and inject version
   cp index.html _headers public/
   sed "s/__VERSION__/v$VERSION/g" komment-embed.js > public/komment-embed.js
   cp -r pkg public/
   ```

3. **Deploy with Wrangler**:
   ```bash
   cd worker
   wrangler deploy
   ```

## Environment Variables (Secrets)

For security, GitHub credentials are not stored in the configuration file. You must set them as secrets:

```bash
cd worker
npx wrangler secret put GITHUB_CLIENT_ID
npx wrangler secret put GITHUB_CLIENT_SECRET
```

## Troubleshooting

- **404 Errors**: Ensure your `wrangler.toml` has the correct assets directory path (`../public`).
- **500 Errors**: Check your worker logs with `wrangler tail`. Most 500 errors in the worker are related to missing secrets or incorrect GitHub App permissions.
- **WASM Load Failures**: Ensure the `pkg/` folder is in the same directory as `komment-embed.js`. The script uses relative paths to find the WASM binary.
- **CORS Issues**: Check the `_headers` file in the `public/` directory. It should allow cross-origin requests for the script and WASM assets.
