import { createEffect, createSignal } from "solid-js";

export function GuestlogList() {
    const [guestlogs, setGuestlogs] = createSignal([]);
    createEffect(async () => {
        const res = await fetch("/api/guestlog");
        const text = await res.text();
        console.log(text);
    }, []);
    return (
        <div class="grid grid-cols-1 gap-5">
            {guestlogs().map((guestlog: IGuestlogEntry) => (
                <div class="bg-gray-100 p-5 rounded-md">
                    <div class="flex justify-between">
                        <h2>{guestlog.fields.name}</h2>
                        <p>{guestlog.fields.Email}</p>
                    </div>
                    <p>{guestlog.fields.Message}</p>
                </div>
            ))}
        </div>
    );
}

interface IGuestlogEntry {
    id: string;
    createdTime: string;
    fields: {
        name: string;
        Email: string;
        Message: string;
    };
}