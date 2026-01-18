import express from 'express';
import Redis from 'ioredis';

const app = express();
const redis = new Redis(process.env.REDIS_HOST || 'localhost');

app.use(express.json());

app.post('/order', async (req, res) => {
  const workerId = process.env.HOSTNAME || 'unknown';

  // Atomic Lua script: check stock and decrement in one operation
  const script = `
    local stock = redis.call('GET', KEYS[1])
    if stock and tonumber(stock) > 0 then
      redis.call('SET', KEYS[1], stock - 1)
      return {1, stock - 1}
    else
      return {0, stock}
    end
  `;

  const [success, remaining] = await redis.eval(script, 1, 'inventory');

  if (success === 1) {
    console.log(`[${workerId}] Order fulfilled. Remaining: ${remaining}`);
    res.status(200).json({ success: true, remaining });
  } else {
    console.log(`[${workerId}] Order rejected. Stock: ${remaining}`);
    res.status(429).json({ success: false, message: 'Out of stock' });
  }
});

app.listen(8888, () => {
  console.log('API server listening on port 8888');
});
