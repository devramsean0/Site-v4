import { defineCollection, z } from 'astro:content';
import { airtableLoader } from '@ascorbic/airtable-loader';
import { s3Loader } from './lib/s3-loader';

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


// Projects tables
const favourite_projects = defineCollection({
     loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "projects",
        queryParams: {
            view: "favourite",
        }
    }),
});

const projects = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "projects",
        queryParams: {
            view: "all",
        }
    }),
});

const technologies = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "technologies",
    }),
});

const gallery = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "gallery_images",
    }),
});

const gallery_photos = defineCollection({
    loader: s3Loader({
        endpoint: import.meta.env.S3_ENDPOINT,
        bucket: "sean-photos-public",
        auth: {
            key_id: import.meta.env.S3_KEY_ID,
            access_key: import.meta.env.S3_ACCESS_KEY
        }
    }),
    schema: z.object({
        data: z.object({}),
    }),
})
// Misc Collections
const kv = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "kv",
    }),
});

const socials = defineCollection({
    loader: airtableLoader({
        base: import.meta.env.AIRTABLE_BASE,
        table: "socials",
    }),
});

export const collections = { guestlog, experience, experience_companies, education, education_providers, favourite_projects, projects, technologies, gallery, gallery_photos, kv, socials };