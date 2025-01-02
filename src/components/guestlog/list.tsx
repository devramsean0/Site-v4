import { createEffect, createSignal } from "solid-js";
import BlankAvatar from "../../assets/no-avatar.jpg";

export function GuestlogList(props: {guestlog: any}) {
    const [guestlogs, setGuestlogs] = createSignal(props.guestlog);
    const fetch_guestlogs = async () => {
        const res = await fetch("/api/guestlog");
        const text = await res.text();
        setGuestlogs(JSON.parse(text));
    }
    createEffect(async () => {
        if (import.meta.env.PROD) {
            await fetch_guestlogs();
        }
    }, []);
    // Attach event listener to cause reload when new guestlog is submitted
    document.addEventListener('guestlog:submitted', async () => {
       await fetch_guestlogs();
    });
    return (
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 pt-4 gap-5 text-black">
            {guestlogs().map((guestlog: any) => (
                <div class="bg-gray-100 p-5 rounded-md">
                    <div class="flex gap-5 items-center">
                        <img src={guestlog.gravatar_url ? `${guestlog.gravatar_url}?s=80&d=identicon` : BlankAvatar.src} alt={guestlog.gravatar_url ? guestlog.name : "Placeholder User Avatar"} class="rounded-full w-20 h-20" loading="lazy"/>
                        <div class="flex flex-col gap-3 justify-between truncate">
                            <h2>{guestlog.name}</h2>
                            <p class="hover:overflow-x-auto">{guestlog.message}</p>
                        </div>
                    </div>
                </div>
            ))}
        </div>
    );
}