import { Hono } from 'hono';
import { serve } from '@hono/node-server';

const app = new Hono();

app.get('/', (c) => {
  return c.text('Hello from Hono! This app is running on Partiri.');
});

app.get('/health', (c) => {
  return c.json({ status: 'ok' });
});

const port = Number(process.env.PORT) || 10000;

serve({ fetch: app.fetch, port }, () => {
  console.log(`Server running on port ${port}`);
});
