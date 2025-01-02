import type { APIRoute } from "astro";
import { getPlayer } from "../../lib/spotify";

export const GET: APIRoute = async ({ request }) => {
    const player = await getPlayer();

    if (player.status === 204 || player.status > 400) {
        console.log('Fetching failed due to status', player.status, await player.json());
        return new Response(JSON.stringify({ playing: false }));
    }
    const song = await player.json();
    if (!song.is_playing) {
        console.log('Nothing playing', player.status);
        return new Response(JSON.stringify({ playing: false }));
    }
    const isPlaying = song.is_playing;
    const title = song.item.name;
    const artist = song.item.artists.map((_artist: any) => _artist.name).join(', ');
    const album = song.item.album.name;
    const albumImageUrl = song.item.album.images[0].url;
    const songUrl = song.item.external_urls.spotify;
    const device = song.device.name;

    return new Response(JSON.stringify({
        playing: isPlaying,
        title,
        artist,
        album,
        albumImageUrl,
        songUrl,
        device
    }));
};