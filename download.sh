#!/bin/bash
set -euo pipefail

# Check if a URL was provided
if [ $# -eq 0 ]; then
  echo "Please provide a URL to a Tiny Desk concert"
  echo "Usage: ./download.sh <URL>"
  exit 1
fi

URL="$1"

# Scrape the set list
echo "Scraping set list..."
npx ts-node scraper.ts "$URL"

# Download the video using yt-dlp
echo "Downloading video..."
yt-dlp "$URL" -o "%(title)s.%(ext)s"

echo "Done!"
