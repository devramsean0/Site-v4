import type { APIRoute } from "astro";

export const GET: APIRoute = async ({ request }) => {
    const res = await fetch(`https://api.airtable.com/v0/${import.meta.env.AIRTABLE_BASE}/guestlog?view=active`, {
        headers: {
            Authorization: `Bearer ${import.meta.env.AIRTABLE_TOKEN}`,
        },
    });
    const resJSON = await res.json();
    const guestlog = resJSON.records.map((entry: any) => {
        delete entry.fields.email;
        return {
            ...entry.fields
        };
    });
    return new Response(JSON.stringify(guestlog), {});
};