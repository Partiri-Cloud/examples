# hello-nestjs

A minimal NestJS 11 application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- NestJS 11 with a single controller handling two routes
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
   cd examples/node/hello-nestjs
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
npm run start:dev
```

The dev server starts on port 3000 by default. Set the `PORT` environment variable to use a different port.

```bash
PORT=8080 npm run start:dev
```

## Learn More
- [Deploying NestJS on Partiri](https://partiri.com/docs/frameworks/nestjs)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
