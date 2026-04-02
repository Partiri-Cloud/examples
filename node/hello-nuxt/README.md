# hello-nuxt

A minimal Nuxt 3 application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Nuxt 3 with a single page component and a server route
- Health check at `/health`
- Multi-stage production Dockerfile

## Deploy to Partiri

### Prerequisites
- [Partiri CLI](https://partiri.com/docs/cli) installed
- A Partiri account with a workspace and project

### Steps
1. Clone and navigate to this example
   ```bash
   git clone https://github.com/partiri-cloud/examples.git
   cd examples/node/hello-nuxt
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

The dev server starts on port 3000 by default. Set the `PORT` environment variable to use a different port.

```bash
PORT=8080 npm run dev
```

## Learn More
- [Deploying Nuxt on Partiri](https://partiri.com/docs/frameworks/nuxt)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
