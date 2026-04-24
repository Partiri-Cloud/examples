# hello-rails

A minimal Rails application ready to deploy on [Partiri](https://partiri.cloud).

## What's Included
- Rails 8 API-only app with a single HTTP endpoint
- Health check at `/health`
- Production Dockerfile
- Listens on `PORT` (default `10000`, matches the Partiri platform default)

## Deploy via the Partiri Dashboard

The Partiri service form has two modes. Choose one.

### Option A — Repository mode (deploy from Git)

1. Fork [partiri-cloud/examples](https://github.com/partiri-cloud/examples) or push this example to your own repo.
2. In the [Partiri dashboard](https://partiri.cloud), click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-rails` |
   | Project | *your project* |
   | Source | **Repository** |
   | Runtime | `ruby` |
   | Repository URL | `https://github.com/<you>/examples` |
   | Branch | `main` |
   | Root directory | `ruby/hello-rails` |
   | Build command | `bundle install --without development test` |
   | Run command | `bundle exec rails server -b 0.0.0.0 -p $PORT` |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

3. Click **Create**. The platform builds and deploys automatically.

### Option B — Registry mode (deploy a pre-built image)

1. Build and push the image to your own registry:
   ```bash
   docker build -t ghcr.io/<you>/hello-rails:latest .
   docker push ghcr.io/<you>/hello-rails:latest
   ```
2. In the Partiri dashboard, click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-rails` |
   | Project | *your project* |
   | Source | **Registry image** |
   | Registry URL | `ghcr.io/<you>/hello-rails:latest` |
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
cd examples/ruby/hello-rails
partiri init
partiri service create
partiri service deploy
```

## Local Development

```bash
bundle install
bundle exec rails server -b 0.0.0.0 -p 10000
```

The app will be available at `http://localhost:10000`. Override with `-p <port>` as needed.

## Learn More
- [Deploying Ruby on Partiri](https://partiri.cloud/documentation/frameworks#ruby)
- [Partiri CLI Reference](https://partiri.cloud/documentation/cli)
