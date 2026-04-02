FROM node:24-slim

WORKDIR /app

COPY package.json ./

RUN npm install --omit=dev && npm cache clean --force

COPY app.js ./

USER 1001

EXPOSE 3000

CMD ["node", "app.js"]
