# hello-rails

A minimal Rails application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Rails 8 API-only app with a single HTTP endpoint
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
   cd examples/ruby/hello-rails
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

Install dependencies and start the server:

```bash
bundle install
rails server -b 0.0.0.0 -p 3000
```

The app will be available at http://localhost:3000.

## Learn More
- [Deploying Rails on Partiri](https://partiri.com/docs/frameworks/rails)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
