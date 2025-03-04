# CLAUDE.md - Guidelines for the Tiny Desk Concert Downloader

## Build & Development Commands
- Setup: `npm install && npx playwright install` - Install dependencies
- Build: `npm run build` - Compile TypeScript
- Run scraper: `npx playwright your-script.ts <URL>` - Run the web scraper with Playwright
- Download video: `yt-dlp <URL>` - Download a video with yt-dlp

## Project Structure
- TypeScript for web scraping (Playwright)
- Bash scripts for orchestration
- Downloads videos using yt-dlp

## Code Style Guidelines
- TypeScript: Use strict typing with interfaces for data structures
- Variable naming: camelCase for variables and functions
- Error handling: Try/catch blocks with descriptive error messages
- Imports: Group imports by type (node modules first, then local modules)
- Formatting: 2-space indentation, trailing commas for multiline arrays/objects
- Comments: JSDoc style for functions, inline comments for complex logic
- Async/await preferred over direct Promise handling

## Testing
- Manual testing by running on various NPR Tiny Desk Concert URLs
- Check output files for correct song listings
