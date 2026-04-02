# hello-svelte

A minimal SvelteKit application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- SvelteKit 2 with a single page route and a server-side health endpoint
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
   cd examples/node/hello-svelte
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

The app will be available at `http://localhost:5173`.

To run the production build locally:

```bash
npm run build
node build/index.js
```

## Learn More
- [Deploying SvelteKit on Partiri](https://partiri.com/docs/frameworks/sveltekit)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
