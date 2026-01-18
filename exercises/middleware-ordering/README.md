# middleware-ordering

## Goal
Fix a middleware ordering bug that prevents Express from parsing JSON request bodies. The application appears to work correctly, but all POST requests receive empty data objects instead of the actual payload.

## The Mission
1. **Open the directory:** Open the `project/` directory in your terminal.
2. **Install dependencies:** Run `pnpm install`.
3. **Initialize Claude Code:** Run the command `claude`.
4. **Trigger Plan Mode:** Press `Shift+Tab` to enter **Plan Mode**, then paste:

> Analyze why POST requests to the /api/echo endpoint return empty data objects. The server should echo back the JSON payload but returns `{ "status": "success", "data": {} }` instead. Find the bug, fix it, and verify the fix works by running the test script.
