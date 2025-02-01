import { defineCollection, z } from 'astro:content';
import { airtableLoader } from '@ascorbic/airtable-loader';
import { glob } from 'astro/loaders';
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

// Blog Collections
const blog_posts = defineCollection({
    loader: glob({
        pattern: "**/*.md",
        base: "content/blog"
    }),
    schema: z.object({
        title: z.string(),
        description: z.string(),
        published_at: z.optional(z.date()),
        tags: z.optional(z.array(z.string()))
    })
});
export const collections = { guestlog, experience, experience_companies, education, education_providers, favourite_projects, projects, technologies, kv, socials, blog_posts };