# hello-gin

A minimal [Gin](https://gin-gonic.com) application ready to deploy on [Partiri](https://partiri.cloud).

## What's Included
- Gin with a single HTTP endpoint
- Health check at `/health`
- Production Dockerfile with multi-stage build
- Listens on `PORT` (default `10000`, matches the Partiri platform default)

## Deploy via the Partiri Dashboard

The Partiri service form has two modes. Choose one.

### Option A — Repository mode (deploy from Git)

1. Fork [partiri-cloud/examples](https://github.com/partiri-cloud/examples) or push this example to your own repo.
2. In the [Partiri dashboard](https://partiri.cloud), click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-gin` |
   | Project | *your project* |
   | Source | **Repository** |
   | Runtime | `go` |
   | Repository URL | `https://github.com/<you>/examples` |
   | Branch | `main` |
   | Root directory | `go/hello-gin` |
   | Build command | `go build -o server .` |
   | Run command | `./server` |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**. The platform builds and deploys automatically.

### Option B — Registry mode (deploy a pre-built image)

1. Build and push the image to your own registry:
   ```bash
   docker build -t ghcr.io/<you>/hello-gin:latest .
   docker push ghcr.io/<you>/hello-gin:latest
   ```
2. In the Partiri dashboard, click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-gin` |
   | Project | *your project* |
   | Source | **Registry image** |
   | Registry URL | `ghcr.io/<you>/hello-gin:latest` |
   | Credentials | *a registry secret* (required for private images) |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**.

> No `Port` field is needed — the platform injects `PORT=10000` and this example listens on it.
>
> Other environment variables are optional — add them in the Environment Variables section as needed.

## Deploy via the CLI

```bash
git clone https://github.com/partiri-cloud/examples.git
cd examples/go/hello-gin
partiri init
partiri service create
partiri service deploy
```

## Local Development

```bash
go mod tidy
go run .
```

The server listens on port **10000** by default. Override with the `PORT` env var:

```bash
PORT=8080 go run .
```

## Learn More
- [Deploying Go on Partiri](https://partiri.cloud/documentation/frameworks#go)
- [Partiri CLI Reference](https://partiri.cloud/documentation/cli)
