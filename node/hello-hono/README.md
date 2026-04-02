# hello-hono

A minimal Hono application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Hono 4 with `@hono/node-server` adapter for two HTTP routes
- Health check at `/health`
- Production Dockerfile (single-stage, no build step required)

## Deploy to Partiri

### Prerequisites
- [Partiri CLI](https://partiri.com/docs/cli) installed
- A Partiri account with a workspace and project

### Steps
1. Clone and navigate to this example
   ```bash
   git clone https://github.com/partiri-cloud/examples.git
   cd examples/node/hello-hono
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

```bash
npm install
npm run dev
```

The app will be available at `http://localhost:3000`.

## Learn More
- [Deploying Hono on Partiri](https://partiri.com/docs/frameworks/hono)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
