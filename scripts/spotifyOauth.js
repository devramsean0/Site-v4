//
// Script generated by GPT4-O because I'm lazy and don't want to write it myself.
//

const https = require('https');
const querystring = require('querystring');

// Replace these with your Spotify Developer app details
const CLIENT_ID = process.env.SPOTIFY_CLIENT_ID;
const CLIENT_SECRET = process.env.SPOTIFY_CLIENT_SECRET;
const REDIRECT_URI = 'http://localhost:3000';

// Step 1: Generate the authorization URL
const authUrl = `https://accounts.spotify.com/authorize?` +
    querystring.stringify({
        client_id: CLIENT_ID,
        response_type: 'code',
        redirect_uri: REDIRECT_URI,
        scope: 'user-read-playback-state', // Get the user's playback state
    });

console.log("1. Open this URL in your browser to authorize the app:");
console.log(authUrl);

// Create a basic server to capture the authorization code
const http = require('http');
const server = http.createServer((req, res) => {
    const url = new URL(req.url, `http://${req.headers.host}`);
    const authorizationCode = url.searchParams.get('code');

    if (authorizationCode) {
        console.log("2. Authorization code received! Requesting access and refresh tokens...");

        // Step 2: Exchange the authorization code for access and refresh tokens
        const tokenData = querystring.stringify({
            grant_type: 'authorization_code',
            code: authorizationCode,
            redirect_uri: REDIRECT_URI,
            client_id: CLIENT_ID,
            client_secret: CLIENT_SECRET,
        });

        const tokenOptions = {
            hostname: 'accounts.spotify.com',
            path: '/api/token',
            method: 'POST',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
                'Content-Length': tokenData.length,
            },
        };

        const tokenRequest = https.request(tokenOptions, (tokenResponse) => {
            let data = '';

            tokenResponse.on('data', (chunk) => {
                data += chunk;
            });

            tokenResponse.on('end', () => {
                const response = JSON.parse(data);
                console.log("Access Token:", response.access_token);
                console.log("Refresh Token:", response.refresh_token);

                // Respond to the browser
                res.writeHead(200, { 'Content-Type': 'text/plain' });
                res.end("Tokens received! You can close this page.");

                // Stop the server
                server.close();
            });
        });

        tokenRequest.write(tokenData);
        tokenRequest.end();
    } else {
        // Handle invalid or missing authorization code
        res.writeHead(400, { 'Content-Type': 'text/plain' });
        res.end("Authorization code not found. Please try again.");
    }
});

// Start the server and listen on a specific port
server.listen(3000, () => {
    console.log("3. Waiting for Spotify authorization...");
});
