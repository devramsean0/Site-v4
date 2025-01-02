import { createEffect, createSignal } from "solid-js";

export function Spotify() {
    const [playing, setPlaying] = createSignal(false);
    const [data, setData] = createSignal<ISpotifyReturnProps>({
        playing: false,
        title: "",
        artist: "",
        album: "",
        albumImageUrl: "",
        songUrl: "",
        device: ""
    });
    createEffect(() => {
        fetch("/api/spotify")
            .then((res) => res.json())
            .then((res) => {
                setPlaying(res.playing);
                setData(res);
            });
    });
    return(
        <div class="[&>*]:text-lg [&>*]:pb-2 [&>*]:pl-2">
        {playing()
            ? <p>I'm currently listening to <a href={data().songUrl}>{data().title} by {data().artist}</a> on {data().device}</p>
            : <p>I'm not listening to anything rn</p>
        }
        </div>
    )
}

interface ISpotifyReturnProps {
    playing: boolean;
    title: string;
    artist: string;
    album: string;
    albumImageUrl: string;
    songUrl: string;
    device: string;
}