# hello-static

A minimal static site ready to deploy on [Partiri](https://partiri.com). Served by Caddy.

## What's Included
- Plain HTML and CSS — no build step required
- Health check at `/health` via Caddy
- Production Dockerfile (non-root UID 1001)

## Deploy to Partiri

### Prerequisites
- [Partiri CLI](https://partiri.com/docs/cli) installed
- A Partiri account with a workspace and project

### Steps
1. Clone and navigate to this example
   ```bash
   git clone https://github.com/partiri-cloud/examples.git
   cd examples/static/hello-static
   ```

2. Initialize your Partiri config
   ```bash
   partiri init
   ```

3. Create and deploy the service
   ```bash
   partiri service create
   partiri service deploy
   ```

## Local Development

**Requirements:** [Caddy](https://caddyserver.com/docs/install)

```bash
caddy run
```

The server starts on port 3000 by default. Set the `PORT` environment variable to override:

```bash
PORT=8080 caddy run
```

Alternatively, open `index.html` directly in your browser — no server needed for basic previewing.

Endpoints:
- `GET /` — the static HTML page
- `GET /health` — `{"status":"ok"}`

## Learn More
- [Deploying Static Sites on Partiri](https://partiri.com/docs/frameworks/static)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
