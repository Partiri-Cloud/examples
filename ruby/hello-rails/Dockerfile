FROM ruby:3.3-slim-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Gemfile Gemfile.lock* ./
RUN bundle install --without development test

COPY . .

RUN useradd -u 1001 -m appuser
USER 1001

ENV RAILS_ENV=production
ENV PORT=3000

EXPOSE 3000

CMD bundle exec rails server -b 0.0.0.0 -p ${PORT}
