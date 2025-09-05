import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
    static override targets = ["output"];
    declare readonly hasOutputTarget: boolean
    declare readonly outputTarget: HTMLParagraphElement

    override async connect() {
        const res = await fetch("/api/spotify");

        switch (res.status) {
            case 200:
                const resJSON: SpotifyNowPlaying = await res.json();
                let songname = `${resJSON.title} by ${resJSON.artists.join(", ")}`
                const img: HTMLImageElement = document.createElement("img");
                img.src = resJSON.album_image_url;
                img.alt = songname;
                img.width = 128;
                img.height = 128;

                const paragraph: HTMLParagraphElement = document.createElement("p");
                paragraph.innerText = "I'm listening to: ";


                const link: HTMLAnchorElement = document.createElement("a");
                link.href = resJSON.song_url;
                link.innerText = songname;
                paragraph.appendChild(link);

                const div: HTMLDivElement = document.createElement("div")
                div.className = "flex items-center gap-2 m-2";
                div.appendChild(img);
                div.appendChild(paragraph);

                this.outputTarget.appendChild(div);
                break;
            case 204:
                this.outputTarget.innerText = "I'm not playing anything";
                break;
            default:
                this.outputTarget.innerText = "Unknown Server Error";
                break;
        }
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