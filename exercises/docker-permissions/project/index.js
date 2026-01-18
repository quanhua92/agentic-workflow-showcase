const express = require('express');
const fs = require('fs');

const app = express();

app.get('/', (_, res) => res.send('Try POST /log'));

app.post('/log', (_, res) => {
  try {
    fs.mkdirSync('./data/logs', { recursive: true });
    fs.writeFileSync('./data/logs/startup.log', `Log: ${new Date().toISOString()}\n`);
    res.send('Success!');
  } catch (err) {
    res.status(500).send(`Error: ${err.message}`);
  }
});

app.listen(8888, () => console.log('Server running on port 8888'));
