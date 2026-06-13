#!/bin/sh
set -e

# Start backend in background
echo "Starting backend on port ${PORT:-8000}..."
/app/backend &
BACKEND_PID=$!

# Small pause to let backend initialise
sleep 1

# Start Discord bot in background
echo "Starting Discord bot..."
/app/discord-bot &
BOT_PID=$!

# Forward signals to both processes
trap "kill $BACKEND_PID $BOT_PID 2>/dev/null; exit" SIGTERM SIGINT

# Wait for either process to exit
wait $BACKEND_PID $BOT_PID
