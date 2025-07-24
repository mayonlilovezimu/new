const express = require('express');
const app = express();
const PORT = 3000;

let openLogs = [];

app.get('/pixel', (req, res) => {
  const email = req.query.email || 'unknown@example.com';
  const time = new Date().toISOString();
  openLogs.push({ email, time });
  console.log(`Открытие: ${email} в ${time}`);

  const pixel = Buffer.from(
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8Xw8AAusB9VN12G0AAAAASUVORK5CYII=',
    'base64'
  );

  res.set('Content-Type', 'image/png');
  res.send(pixel);
});

app.get('/opens', (req, res) => {
  res.json(openLogs);
});

app.listen(PORT, () => {
  console.log(`Сервер запущен: http://menu-yes.shop:${PORT}`);
});