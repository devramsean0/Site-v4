import type { APIRoute } from "astro";

export const GET: APIRoute = async ({ request }) => {
    console.log(process.env.PUBLIC_AIRTABLE_API)
    const res = await fetch("https://api.airtable.com/v0/appjAFKlWvVpwaM7k/Guestlog?view=Active", {
        headers: {
            Authorization: `Bearer ${process.env.AIRTABLE_API}`,
        },
    });
    const resJSON = await res.json();
    console.log(resJSON)
    return new Response(JSON.stringify(resJSON.records), {});
};