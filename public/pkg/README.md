# komment

A high-performance commenting system powered by **Rust**, **WebAssembly (WASM)**, and **GitHub Discussions**.

Komment provides a secure, fast, and modern way to add discussions to your website without managing a database. It mirrors the core functionality of [giscus](https://giscus.app) but is built entirely with Rust on both the client and the server.

## Features

- **Blazing Fast**: Powered by Rust compiled to WASM.
- **No Database**: Uses GitHub Discussions as the data store.
- **Zero-Config Styling**: All CSS is bundled in the script; just add the container.
- **Portable Script**: Load `komment-embed.js` from any domain or CDN.
- **Multi-Domain Support**: A centralized OAuth callback allows one deployment to serve multiple sites.
- **Full CRUD**: Post, Edit, and Delete comments with a modern icon-based UI.
- **Automatic Provisioning**: Automatically creates threads for new pages with a reference link to the source.

## Quick Start (Embedded)

To use `komment` on any website, simply add the following:

```html
<!-- 1. The container -->
<div class="komment"></div>

<!-- 2. Load and Initialize -->
<script type="module">
  // Load the script from your deployed worker
  import "https://your-worker.workers.dev/komment-embed.js";
  
  // Initialize with your repo and config
  komment('your-username/your-repo', {
    clientId: 'your-github-client-id' // From your GitHub App
  });
</script>
```

## Setup & Deployment

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) v0.15.0+
- [worker-build](https://github.com/cloudflare/workers-rs) (`cargo install worker-build`)
- [Cloudflare Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- [Just](https://github.com/casey/just) (optional, but recommended for build commands)

### 1. Build and Deploy
The project uses a `justfile` for easy management.

```bash
# Rebuild WASM and deploy everything to Cloudflare
just deploy
```

### 2. Configure GitHub App
1. Create a [GitHub App](https://github.com/settings/apps/new).
2. Set **Callback URL** to your worker's callback endpoint:
   `https://your-worker.workers.dev/api/auth/callback`
3. Under **Permissions**, set **Repository permissions** -> **Discussions** to `Read & write`.
4. Enable **Discussions** in your target repository settings.

### 3. Set Secrets
Run these in the `worker/` directory:
```bash
cd worker
npx wrangler secret put GITHUB_CLIENT_ID
npx wrangler secret put GITHUB_CLIENT_SECRET
```

## Documentation

- [**HOW-TO-USE.md**](./doc/HOW-TO-USE.md): Step-by-step setup guide.
- [**DESIGN.md**](./doc/DESIGN.md): Internal architecture and design decisions.
- [**CLOUDFLARE.md**](./doc/CLOUDFLARE.md): Worker-specific deployment details.
- [**SECURITY.md**](./doc/SECURITY.md): Details on how secrets and tokens are handled.

## License

Dual-licensed under [MIT](./LICENSE-MIT) and [Apache 2.0](./LICENSE-APACHE).
Copyright (c) 2026 Seungjin Kim.
