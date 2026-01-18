# docker-permissions

## Goal
Fix a Docker container permission error that prevents the application from writing files to disk.

## The Mission
1. **Open the directory:** Open the `project/` directory in your terminal.
2. **Initialize Claude Code:** Run the command `claude`.
3. **Trigger Plan Mode:** Press `Shift+Tab` to enter **Plan Mode**, then paste:

> Build the Docker container, run it, and test the application by curling the endpoint. The container starts and the Express server runs, but the POST /log endpoint fails with "EACCES: permission denied". Perform a system analysis to identify why the write operation is failing, propose a plan to fix it, and verify the fix works.
