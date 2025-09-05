import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
    static override targets = ["output"];
    declare readonly hasOutputTarget: boolean
    declare readonly outputTarget: HTMLParagraphElement

    override async connect() {
        const res = await fetch("/api/spotify");
        let output: string;

        switch (res.status) {
            case 200:
                const resJSON: SpotifyNowPlaying = await res.json();
                output = `I'm currently listening to <a href="${resJSON.song_url}">${resJSON.title} by ${resJSON.artists.join(", ")}</a> on ${resJSON.device}`;
                break;
            case 204:
                output = "I'm not playing anything";
                break;
            default:
                output = "Unknown Server Error";
                break;
        }
        this.outputTarget.innerHTML = output;
    }
}

interface SpotifyNowPlaying {
    is_playing: boolean,
    title: string,
    artists: string[],
    album: string,
    album_image_url: string,
    song_url: string,
    device: string
}