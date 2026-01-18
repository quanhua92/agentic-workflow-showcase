# distributed-race-condition

## Goal
Fix an inventory management bug that allows more items to be sold than are actually available.

## The Mission
1. **Open the directory:** Open the `project/` directory in your terminal.
2. **Initialize Claude Code:** Run the command `claude`.
3. **Trigger Plan Mode:** Press `Shift+Tab` to enter **Plan Mode**, then paste:

> Start the microservices system with Docker Compose. The application consists of 3 API replicas and a Redis service. Seed the inventory with 1 item, then run the hammer-test script to fire 10 concurrent requests. Even though only 1 item is available, multiple orders succeed and customers receive order confirmations. First, run docker compose to verify and confirm you have identified the root cause of the bug. Then and only then, propose a plan to fix it and verify the fix works.

