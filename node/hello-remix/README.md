# hello-remix

A minimal Remix application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Remix 2 with React 19, served via `@remix-run/serve`
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
   cd examples/node/hello-remix
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
npm start
```

## Learn More
- [Deploying Remix on Partiri](https://partiri.com/docs/frameworks/remix)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
