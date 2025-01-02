# Site-V4
The like 8th revison of my personal site.

## Key Technolgies
- Astro
- Solidjs
- TailwindCSS
- Airtable

## Local development?
Just don't. Everything needs access to an airtable base that *probably* contains PII.
### But what if I want to?
1. Figure out the airtable schema (have fun)
2. Run `bun install`
2. Create yourself a spotify oauth client
3. Rename to .env then fill out .env.example (Use scripts/spotifyOauth.js to get the refresh token)
4. Run `bun dev`
5. Success
