#!/bin/bash

curl -X POST http://localhost:8888/api/echo \
  -H "Content-Type: application/json" \
  -d '{"agentic": "enabled"}'
