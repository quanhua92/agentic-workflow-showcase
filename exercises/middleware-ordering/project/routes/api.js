import express from 'express';

const router = express.Router();

router.post('/echo', (req, res) => {
  console.log('[Route] Received body:', req.body);
  res.json({
    status: 'success',
    data: req.body
  });
});

export default router;
