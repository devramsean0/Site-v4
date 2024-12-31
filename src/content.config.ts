import { defineCollection, z } from 'astro:content';
import { airtableLoader } from '@ascorbic/airtable-loader';

const guestlog = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "guestlog",
        queryParams: {
            view: "active",
        }
    }),
});
export const collections = { guestlog };