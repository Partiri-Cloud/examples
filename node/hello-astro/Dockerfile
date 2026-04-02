# syntax=docker/dockerfile:1

# ---- build stage ----
FROM node:24-slim AS build

WORKDIR /app

COPY package.json ./
RUN npm install

COPY . .
RUN npm run build

# ---- runtime stage ----
FROM node:24-slim AS runtime

WORKDIR /app

COPY --from=build /app/dist ./dist
COPY package.json ./

# Install only production dependencies for the runtime image
RUN npm install --omit=dev

# Non-root user (UID 1001) — matches Partiri K8s security context
RUN groupadd --gid 1001 appgroup && \
    useradd --uid 1001 --gid appgroup --shell /bin/sh --no-create-home appuser

USER 1001

ENV NODE_ENV=production
ENV PORT=3000

EXPOSE 3000

CMD ["node", "dist/server/entry.mjs"]
