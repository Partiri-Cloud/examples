# syntax=docker/dockerfile:1

# ---- build stage ----
FROM node:24-slim AS build

WORKDIR /app

COPY package.json ./
RUN npm install

COPY . .
RUN npm run build

# Remove dev dependencies after build
RUN npm prune --omit=dev

# ---- runtime stage ----
FROM node:24-slim AS runtime

WORKDIR /app

# Copy only the built output and production node_modules
COPY --from=build /app/build ./build
COPY --from=build /app/node_modules ./node_modules
COPY package.json ./

# Non-root user (UID 1001) — matches Partiri K8s security context
RUN groupadd --gid 1001 appgroup && \
    useradd --uid 1001 --gid appgroup --shell /bin/sh --no-create-home appuser

USER 1001

ENV NODE_ENV=production
ENV PORT=10000

EXPOSE 10000

CMD ["node", "build/index.js"]
