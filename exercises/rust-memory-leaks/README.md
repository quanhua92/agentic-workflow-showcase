# rust-memory-leaks

## Goal
Identify and fix a logical memory leak in Rust that bypasses the borrow checker and causes a runtime crash.

## Why this is a Senior Challenge

* **Invisible to Compiler:** The borrow checker is satisfied. This is a logic bug, not a safety bug.
* **Observational Debugging:** You cannot just "read" the error log. You have to observe the process behavior (RAM usage) to understand that memory isn't being freed.
* **Architectural Refactoring:** The fix isn't a one-line change; it requires changing the ownership strategy to properly manage shared state.

## The Mission
1. **Open the directory:** Open the `project/` directory in your terminal.
2. **Initialize Claude Code:** Run the command `claude`.
3. **Trigger Plan Mode:** Press `Shift+Tab` to enter **Plan Mode**, then paste:

> The application crashes under load after consuming excessive memory. Perform a system analysis of the code and runtime behavior. Use available tools to identify the root cause of the resource issue. Propose a plan to refactor the data structures and verify the fix by monitoring resource stability during execution.
