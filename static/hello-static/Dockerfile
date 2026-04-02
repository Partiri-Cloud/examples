FROM caddy:2-alpine

COPY Caddyfile /etc/caddy/Caddyfile
COPY index.html styles.css /srv/

RUN adduser -u 1001 -D -H -s /sbin/nologin appuser \
    && chown -R appuser /srv \
    && chown -R appuser /data \
    && chown -R appuser /config

USER 1001

EXPOSE 3000
