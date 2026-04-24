'use strict';

const express = require('express');

const app = express();
const port = parseInt(process.env.PORT ?? '10000', 10);

app.get('/', (_req, res) => {
  res.send('Hello from Express on Partiri!');
});

app.get('/health', (_req, res) => {
  res.status(200).json({ status: 'ok' });
});

app.listen(port, () => {
  console.log(`Server listening on port ${port}`);
});
