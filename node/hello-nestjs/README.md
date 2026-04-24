# hello-nestjs

A minimal NestJS 11 application ready to deploy on [Partiri](https://partiri.cloud).

## What's Included
- NestJS 11 with a single controller handling two routes
- Health check at `/health`
- Multi-stage production Dockerfile
- Listens on `PORT` (default `10000`, matches the Partiri platform default)

## Deploy via the Partiri Dashboard

The Partiri service form has two modes. Choose one.

### Option A — Repository mode (deploy from Git)

1. Fork [partiri-cloud/examples](https://github.com/partiri-cloud/examples) or push this example to your own repo.
2. In the [Partiri dashboard](https://partiri.cloud), click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-nestjs` |
   | Project | *your project* |
   | Source | **Repository** |
   | Runtime | `node` |
   | Repository URL | `https://github.com/<you>/examples` |
   | Branch | `main` |
   | Root directory | `node/hello-nestjs` |
   | Build command | `npm install && npm run build` |
   | Run command | `node dist/main.js` |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**. The platform builds and deploys automatically.

### Option B — Registry mode (deploy a pre-built image)

1. Build and push the image to your own registry:
   ```bash
   docker build -t ghcr.io/<you>/hello-nestjs:latest .
   docker push ghcr.io/<you>/hello-nestjs:latest
   ```
2. In the Partiri dashboard, click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-nestjs` |
   | Project | *your project* |
   | Source | **Registry image** |
   | Registry URL | `ghcr.io/<you>/hello-nestjs:latest` |
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
cd examples/node/hello-nestjs
partiri init
partiri service create
partiri service deploy
```

## Local Development

```bash
npm install
npm run start:dev
```

The server listens on port **10000** by default. Override with the `PORT` env var:

```bash
PORT=8080 npm run start:dev
```

## Learn More
- [Deploying Node.js on Partiri](https://partiri.cloud/documentation/frameworks#node)
- [Partiri CLI Reference](https://partiri.cloud/documentation/cli)
