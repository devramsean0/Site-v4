import querystring from 'querystring'

const {
  SPOTIFY_CLIENT_ID: client_id,
  SPOTIFY_CLIENT_SECRET: client_secret,
  SPOTIFY_REFRESH_TOKEN: refresh_token
} = import.meta.env;

const basic = Buffer.from(`${client_id}:${client_secret}`).toString('base64');
const PLAYER_ENDPOINT = `https://api.spotify.com/v1/me/player?market=GB`;
const TOKEN_ENDPOINT = `https://accounts.spotify.com/api/token`;

const getAccessToken = async () => {
  const response = await fetch(TOKEN_ENDPOINT, {
    method: 'POST',
    headers: {
      Authorization: `Basic ${basic}`,
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: querystring.stringify({
      grant_type: 'refresh_token',
      refresh_token
    })
  });

  return await response.json();
};

export const getPlayer = async () => {
  const { access_token } = await getAccessToken();

  return fetch(PLAYER_ENDPOINT, {
    headers: {
      Authorization: `Bearer ${access_token}`
    }
  });
};