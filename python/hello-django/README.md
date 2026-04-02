# hello-django

A minimal Django application ready to deploy on [Partiri](https://partiri.com).

## What's Included
- Django 5.2 with a single HTTP endpoint
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
   cd examples/python/hello-django
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
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
SECRET_KEY=local-dev-key PORT=3000 python manage.py runserver 0.0.0.0:3000
```

Visit `http://localhost:3000` and `http://localhost:3000/health`.

## Learn More
- [Deploying Django on Partiri](https://partiri.com/docs/frameworks/django)
- [Partiri CLI Reference](https://partiri.com/docs/cli)
