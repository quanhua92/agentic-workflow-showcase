import express from 'express';
import Redis from 'ioredis';

const app = express();
const redis = new Redis(process.env.REDIS_HOST || 'localhost');

app.use(express.json());

app.post('/order', async (req, res) => {
  const stock = await redis.get('inventory');
  const workerId = process.env.HOSTNAME || 'unknown';

  // Simulate processing latency to widen the race window
  await new Promise(resolve => setTimeout(resolve, 150));

  if (stock > 0) {
    await redis.set('inventory', stock - 1);
    console.log(`[${workerId}] Order fulfilled. Stock: ${stock} -> ${stock - 1}`);
    res.status(200).json({ success: true, remaining: stock - 1 });
  } else {
    console.log(`[${workerId}] Order rejected. Stock: ${stock}`);
    res.status(429).json({ success: false, message: 'Out of stock' });
  }
});

app.listen(8888, () => {
  console.log('API server listening on port 8888');
});
