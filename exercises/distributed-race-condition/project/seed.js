import Redis from 'ioredis';

const redis = new Redis(process.env.REDIS_HOST || 'localhost');

await redis.set('inventory', 1);

console.log('Inventory seeded with 1 item');

process.exit(0);
