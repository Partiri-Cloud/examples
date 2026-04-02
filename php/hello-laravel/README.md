# hello-laravel

A minimal Laravel application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Laravel 12 with a single HTTP endpoint
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
   cd examples/php/hello-laravel
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

Install dependencies, generate an app key, and start the server:

```bash
composer install
cp .env.example .env
php artisan key:generate
php artisan serve --port=3000
```

The app will be available at http://localhost:3000.

## Learn More
- [Deploying Laravel on Partiri](https://partiri.com/docs/frameworks/laravel)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
