# hello-express

A minimal Express 5 application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Express 5 with two HTTP endpoints
- Health check at `/health`
- Production Dockerfile

## Deploy to Partiri

### Prerequisites
- [Partiri CLI](https://partiri.com/docs/cli) installed
- A Partiri account with a workspace and project

### Steps
1. Clone and navigate to this example
   ```bash
   git clone https://github.com/partiri-cloud/examples.git
   cd examples/node/hello-express
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
node app.js
```

The server starts on port 3000 by default. Set the `PORT` environment variable to use a different port.

```bash
PORT=8080 node app.js
```

## Learn More
- [Deploying Express on Partiri](https://partiri.com/docs/frameworks/express)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
