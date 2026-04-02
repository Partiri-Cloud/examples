# hello-astro

A minimal Astro application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Astro 5 in SSR mode with the Node.js adapter
- Health check at `/health`
- Production Dockerfile with multi-stage build

## Deploy to Partiri

### Prerequisites
- [Partiri CLI](https://partiri.com/docs/cli) installed
- A Partiri account with a workspace and project

### Steps
1. Clone and navigate to this example
   ```bash
   git clone https://github.com/partiri-cloud/examples.git
   cd examples/node/hello-astro
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

The app will be available at `http://localhost:4321`.

To run the production build locally:

```bash
npm run build
node dist/server/entry.mjs
```

## Learn More
- [Deploying Astro on Partiri](https://partiri.com/docs/frameworks/astro)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
