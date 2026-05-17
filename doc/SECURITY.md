# Security Guide: Protecting Secrets in Komment

This document explains how Komment handles sensitive information like GitHub API tokens and secrets to ensure your deployment is secure.

## The Challenge
Directly communicating with the GitHub API from a browser requires an access token. If you were to use a Personal Access Token (PAT) with write access in your frontend code, anyone visiting your site could steal it and use it to modify your repository.

## Solution: The Cloudflare Worker Proxy
Komment solves this by using a Cloudflare Worker as a secure middleman.

### How it Works
1. **OAuth Flow**: When a user clicks "Login with GitHub", they are redirected to GitHub's OAuth service. After authorization, they are sent back to your worker's `/api/auth/callback` endpoint with a temporary `code`.
2. **Token Exchange**: The worker receives this `code` and exchanges it for a real `access_token` using your **GitHub App Client Secret**. This secret is stored securely as a Cloudflare Worker Secret and is **never** exposed to the browser.
3. **Frontend Storage**: The worker sends the `access_token` back to the browser, which stores it in `localStorage`.
4. **GraphQL Proxy**: When the WASM widget needs to fetch or post comments, it sends the request to the worker's `/api/graphql` endpoint, including the user's `access_token` in the `Authorization` header. The worker then forwards this request to GitHub.

### Why this is Secure
- **Secret Protection**: Your GitHub App's `Client Secret` is never sent to the browser.
- **Limited Scope**: The user's `access_token` only has the permissions granted by your GitHub App and only for the duration allowed by GitHub.
- **CORS Protection**: The worker proxy allows you to control which domains are permitted to communicate with your backend, preventing unauthorized sites from "hijacking" your comment system.

## Configuration Best Practices

### 1. Use Wrangler Secrets
Always use `wrangler secret put` to set your GitHub credentials. Never hardcode them in `wrangler.toml`.

```bash
npx wrangler secret put GITHUB_CLIENT_ID
npx wrangler secret put GITHUB_CLIENT_SECRET
```

### 2. Restrict GitHub App Permissions
Set your GitHub App's permissions to the absolute minimum required:
- **Repository permissions** -> **Discussions**: `Read & write`.

### 3. Initialize Komment Correctly
In your HTML, only provide the `clientId`:

```javascript
komment('your-username/your-repo', {
  clientId: 'your-github-client-id' // Safe to expose
});
```

The system will automatically find your worker (where the script is hosted) and use it as the proxy for all sensitive operations.

## Summary of Best Practices
1. **Never** use a Personal Access Token (PAT) in the browser.
2. **Always** use the provided Cloudflare Worker proxy.
3. **Regularly** audit your GitHub App's installation and permissions.
4. **Monitor** your Cloudflare Worker logs for any unusual activity.
