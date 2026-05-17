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
   Ensure `public/` contains `index.html`, `komment-embed.js`, and the `pkg/` folder.
   ```bash
   mkdir -p public
   cp index.html komment-embed.js _headers public/
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
