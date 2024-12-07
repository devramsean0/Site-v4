import { defineCollection, z } from 'astro:content';
import { glob, file } from 'astro/loaders';

const experience = defineCollection({
    loader: file("src/data/experience.json"),
    schema: z.object({
        id: z.number(),
        title: z.string(),
        description: z.string(),
        start_date: z.string(),
        end_date: z.string().optional(),
        company: z.array(z.string()),
        type: z.array(z.string()),
    })
})

const companies = defineCollection({
    loader: file("src/data/companies.json"),
    schema: z.object({
        name: z.string(),
        logo_url: z.string(),
        url: z.string(),
    })
})

const education = defineCollection({
    loader: file("src/data/education.json"),
    schema: z.object({
        id: z.number(),
        title: z.string(),
        description: z.string(),
        start_date: z.string(),
        end_date: z.string().optional(),
        provider: z.array(z.string()),
    })
})

const providers = defineCollection({
    loader: file("src/data/providers.json"),
    schema: z.object({
        name: z.string(),
        logo_url: z.string(),
        url: z.string(),
    })
})

export { experience, companies, education, providers };