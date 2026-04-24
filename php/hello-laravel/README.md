# hello-laravel

A minimal Laravel application ready to deploy on [Partiri](https://partiri.cloud).

## What's Included
- Laravel 12 with a single HTTP endpoint
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
   | Name | `hello-laravel` |
   | Project | *your project* |
   | Source | **Repository** |
   | Runtime | `php` |
   | Repository URL | `https://github.com/<you>/examples` |
   | Branch | `main` |
   | Root directory | `php/hello-laravel` |
   | Build command | `composer install --no-dev --optimize-autoloader` |
   | Run command | `php artisan serve --host=0.0.0.0 --port=$PORT` |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

   **Required env var:** add `APP_KEY=<generated-key>` (run `php artisan key:generate --show` locally to produce one) in the Environment Variables section.

3. Click **Create**. The platform builds and deploys automatically.

### Option B — Registry mode (deploy a pre-built image)

1. Build and push the image to your own registry:
   ```bash
   docker build -t ghcr.io/<you>/hello-laravel:latest .
   docker push ghcr.io/<you>/hello-laravel:latest
   ```
2. In the Partiri dashboard, click **Create service** and fill in:

   | Form field | Value |
   |---|---|
   | Name | `hello-laravel` |
   | Project | *your project* |
   | Source | **Registry image** |
   | Registry URL | `ghcr.io/<you>/hello-laravel:latest` |
   | Credentials | *a registry secret* (required for private images) |
   | Health check path | `/health` |
   | Region | *your region* |
   | Pod | *choose a pod size* |

   **Required env var:** add `APP_KEY=<generated-key>` in the Environment Variables section.

3. Click **Create**.

> No `Port` field is needed — the platform injects `PORT=10000` and this example listens on it.
>
> Other environment variables are optional — add them in the Environment Variables section as needed.

## Deploy via the CLI

```bash
git clone https://github.com/partiri-cloud/examples.git
cd examples/php/hello-laravel
partiri init
partiri service create
partiri service deploy
```

## Local Development

```bash
composer install
cp .env.example .env
php artisan key:generate
php artisan serve --host=0.0.0.0 --port=10000
```

The app will be available at `http://localhost:10000`. Override with `--port <port>` as needed.

## Learn More
- [Deploying PHP on Partiri](https://partiri.cloud/documentation/frameworks#php)
- [Partiri CLI Reference](https://partiri.cloud/documentation/cli)
