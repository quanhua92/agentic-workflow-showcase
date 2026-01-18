#!/bin/bash
# Distribute requests across 3 API replicas to trigger distributed race condition
ports=(8888 8889 8890)
for i in {1..10}; do
  port=${ports[$((i % 3))]}
  curl -s -X POST http://localhost:${port}/order &
done
wait
