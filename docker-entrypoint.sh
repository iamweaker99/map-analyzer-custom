#!/bin/sh
set -e

# Start the backend API server in the background
echo "Starting backend on port ${PORT:-8000}..."
/app/backend &
BACKEND_PID=$!

# Start the Discord bot in the background
echo "Starting Discord bot..."
/app/discord-bot &
BOT_PID=$!

# Trap SIGTERM/SIGINT and forward to child processes
trap 'echo "Shutting down..."; kill $BACKEND_PID $BOT_PID 2>/dev/null; wait' TERM INT

# Wait for any process to exit
wait
