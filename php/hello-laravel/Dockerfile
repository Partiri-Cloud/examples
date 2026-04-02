FROM php:8.3-cli

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    unzip \
  && rm -rf /var/lib/apt/lists/*

COPY --from=composer:2 /usr/bin/composer /usr/bin/composer

WORKDIR /app

COPY composer.json ./
RUN composer install --no-dev --optimize-autoloader --no-scripts --no-interaction

COPY . .
RUN composer dump-autoload --optimize

RUN useradd -u 1001 -m appuser
USER 1001

ENV APP_ENV=production
ENV PORT=3000

EXPOSE 3000

CMD php artisan serve --host=0.0.0.0 --port=${PORT}
