# hello-static

A minimal static site ready to deploy on [Partiri](https://partiri.cloud). Served by Caddy (when deployed as a container) or by the Partiri static-site runtime (when deployed with `runtime=static`).

## What's Included
- Plain HTML and CSS — no build step required
- Health check at `/health` (provided by Caddy when using the container path)
- Production Dockerfile (non-root UID 1001)
- Listens on `PORT` (default `10000`, matches the Partiri platform default) when running the container

## Deploy via the Partiri Dashboard

The Partiri service form has two modes. Choose one.

### Option A — Repository mode (deploy from Git)

1. Fork [partiri-cloud/examples](https://github.com/partiri-cloud/examples) or push this example to your own repo.
2. In the [Partiri dashboard](https://partiri.cloud), click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-static` |
   | Project | *your project* |
   | Source | **Repository** |
   | Runtime | `static` |
   | Repository URL | `https://github.com/<you>/examples` |
   | Branch | `main` |
   | Root directory | `static/hello-static` |
   | Build command | *(leave empty — no build step)* |
   | Run command | *(leave empty — static runtime serves files directly)* |
   | Publish directory | `.` |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**. The platform serves the files from the publish directory.

### Option B — Registry mode (deploy a pre-built image)

1. Build and push the image to your own registry:
   ```bash
   docker build -t ghcr.io/<you>/hello-static:latest .
   docker push ghcr.io/<you>/hello-static:latest
   ```
2. In the Partiri dashboard, click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-static` |
   | Project | *your project* |
   | Source | **Registry image** |
   | Registry URL | `ghcr.io/<you>/hello-static:latest` |
   | Credentials | *a registry secret* (required for private images) |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**.

> When deploying the container image, Caddy listens on the `PORT` env var (platform injects `10000`). No `Port` field is needed.
>
> Other environment variables are optional — add them in the Environment Variables section as needed.

## Deploy via the CLI

```bash
git clone https://github.com/partiri-cloud/examples.git
cd examples/static/hello-static
partiri init
partiri service create
partiri service deploy
```

## Local Development

**Requirements:** [Caddy](https://caddyserver.com/docs/install)

```bash
caddy run
```

The server listens on port **10000** by default. Override with the `PORT` env var:

```bash
PORT=8080 caddy run
```

Alternatively, open `index.html` directly in your browser — no server needed for basic previewing.

Endpoints:
- `GET /` — the static HTML page
- `GET /health` — `{"status":"ok"}`

## Learn More
- [Deploying Static Sites on Partiri](https://partiri.cloud/documentation/frameworks#static)
- [Partiri CLI Reference](https://partiri.cloud/documentation/cli)
