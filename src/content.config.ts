import { defineCollection, z } from 'astro:content';
import { airtableLoader } from '@ascorbic/airtable-loader';

// Guestlog tables
const guestlog = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "guestlog",
        queryParams: {
            view: "active",
        }
    }),
});

// Experience tables
const experience = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "experience_positions",
        queryParams: {
            view: "active",
        }
    }),
});
const experience_companies = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "experience_companies"
    }),
});

// Education tables
const education = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "education",
        queryParams: {
            view: "active",
        }
    }),
});

const education_providers = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "education_providers"
    }),
});

export const collections = { guestlog, experience, experience_companies, education, education_providers };