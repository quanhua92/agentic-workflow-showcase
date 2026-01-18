import express from 'express';
import apiRouter from './routes/api.js';

const app = express();

app.use('/api', apiRouter);
app.use(express.json());

app.listen(8888, () => {
  console.log('Server running on port 8888');
});
