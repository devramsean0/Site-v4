import type { APIRoute } from "astro";
import crypto from 'crypto';

export const prerender = false;

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

export const POST: APIRoute = async ({ request }) => {
    const body = await request.json();

    let gravatar_url: string = "";
    if (body.gravatar_enabled) {
        gravatar_url = getGravatarUrl(body.email);
    }
    const reqBody = JSON.stringify({
        records: [
            {
                fields: {
                    name: body.name,
                    email: body.email,
                    message: body.message,
                    active: true,
                    gravatar_url,
                }
            }
        ]
    });
    console.log(reqBody);
    const res = await fetch(`https://api.airtable.com/v0/${import.meta.env.AIRTABLE_BASE}/guestlog`, {
        method: 'POST',
        headers: {
            Authorization: `Bearer ${import.meta.env.AIRTABLE_TOKEN}`,
            'Content-Type': 'application/json',
        },
        body: reqBody
    });
    const resJSON = await res.json();
    console.log(resJSON)
    return new Response(JSON.stringify(resJSON), {});
}

function getGravatarUrl(email: string, size = 80) {
    const trimmedEmail = email.trim().toLowerCase();
    const hash = crypto.createHash('sha256').update(trimmedEmail).digest('hex');
    return `https://www.gravatar.com/avatar/${hash}`;
}