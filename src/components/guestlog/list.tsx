import { createEffect, createSignal } from "solid-js";

export function GuestlogList(props: {guestlog: any}) {
    const [guestlogs, setGuestlogs] = createSignal(props.guestlog);
    createEffect(async () => {
        const res = await fetch("/api/guestlog");
        const text = await res.text();
        console.log(text);
    }, []);
    return (
        <div class="grid grid-cols-4 gap-5 text-black">
            {guestlogs().map((guestlog: any) => (
                <div class="bg-gray-100 p-5 rounded-md">
                    <div class="flex justify-between">
                        <h2>{guestlog.name}</h2>
                        <p>{guestlog.email}</p>
                    </div>
                    <p>{guestlog.message}</p>
                </div>
            ))}
        </div>
    );
}
