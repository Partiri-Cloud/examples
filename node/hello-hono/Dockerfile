# syntax=docker/dockerfile:1

FROM node:24-slim

WORKDIR /app

COPY package.json ./
RUN npm install --omit=dev

COPY app.js ./

# Non-root user (UID 1001) — matches Partiri K8s security context
RUN groupadd --gid 1001 appgroup && \
    useradd --uid 1001 --gid appgroup --shell /bin/sh --no-create-home appuser

USER 1001

ENV NODE_ENV=production
ENV PORT=10000

EXPOSE 10000

CMD ["node", "app.js"]
