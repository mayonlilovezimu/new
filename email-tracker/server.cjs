const express = require('express');
const path = require('path');
const fs = require('fs');

const app = express();
const PORT = 3000;

let openLogs = [];

// Маршрут для пикселя
app.get('/pixel', (req, res) => {
  const email = req.query.email || 'unknown@example.com';
  const time = new Date().toISOString();

  openLogs.push({ email, time });
  console.log(`Письмо открыто: ${email} в ${time}`);

  // Отдаем 1x1 прозрачный PNG пиксель
  res.set('Content-Type', 'image/png');
  res.sendFile(path.join(__dirname, 'pixel.png'));
});

// API для получения списка открытых писем
app.get('/opens', (req, res) => {
  res.json(openLogs);
});

// Запускаем сервер
app.listen(PORT, () => {
  console.log(`Сервер отслеживания открыт на http://localhost:${PORT}`);
});
