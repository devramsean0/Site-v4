import { defineCollection, z } from 'astro:content';
import { glob, file } from 'astro/loaders';

const experience = defineCollection({
    loader: file("src/data/experience.json"),
    schema: z.object({
        id: z.string(),
        createdTime: z.string(),
        fields: z.object({
            id: z.number(),
            title: z.string(),
            description: z.string(),
            start_date: z.string(),
            end_date: z.string().optional(),
            company: z.array(z.string()),
            company_name: z.array(z.string()),
            type: z.string(),
            active: z.boolean(),
        })
    })
})

const companies = defineCollection({
    loader: file("src/data/companies.json"),
    schema: z.object({
        id: z.string(),
        createdTime: z.string(),
        fields: z.object({
            name: z.string(),
            logo_link: z.optional(z.string()),
            link: z.optional(z.string()),
        })
    })
})

const education = defineCollection({
    loader: file("src/data/education.json"),
    schema: z.object({
        id: z.string(),
        createdTime: z.string(),
        fields: z.object({
            id: z.number(),
            title: z.string(),
            description: z.string(),
            start_date: z.string(),
            end_date: z.string().optional(),
            provider: z.array(z.string()),
            provider_name: z.array(z.string()),
        })
    })
})

const providers = defineCollection({
    loader: file("src/data/providers.json"),
    schema: z.object({
        id: z.string(),
        createdTime: z.string(),
        fields: z.object({
            name: z.string(),
            logo_link: z.optional(z.string()),
            link: z.optional(z.string()),
        })
    })
})

const newsletter = defineCollection({
    loader: file("src/data/newsletter.json"),
    schema: z.object({
        id: z.string(),
        createdTime: z.string(),
        fields: z.object({
            id: z.number(),
            title: z.string(),
            contents: z.string(),
            sent: z.boolean(),
        })
    })
})

const blog = defineCollection({
    loader: file("src/data/blog.json"),
    schema: z.object({
        id: z.string(),
        createdTime: z.string(),
        fields: z.object({
            id: z.number(),
            title: z.string(),
            description: z.string(),
            contents: z.string(),
            tags: z.array(z.string()),
            active: z.boolean(),
            comments_enabled: z.boolean(),
            comments: z.array(z.string()).optional(),
        })
    })
})

const projects = defineCollection({
    loader: file("src/data/projects.json"),
    schema: z.object({
        id: z.string(),
        createdTime: z.string(),
        fields: z.object({
            name: z.string(),
            description: z.string(),
            source_control: z.optional(z.string()),
            documentation: z.optional(z.string()),
            demo: z.optional(z.string()),
            preview_image_link: z.optional(z.string()),
            technologies: z.array(z.string()),
            active: z.boolean(),
        })
    })
})

const guestlog = defineCollection({
    loader: file("src/data/guestlog.json"),
    schema: z.object({
        id: z.string(),
        createdTime: z.string(),
        fields: z.object({
            name: z.string(),
            email: z.string(),
            message: z.string(),
            active: z.boolean(),
            gravatar_enabled: z.boolean(),
        })
    })
})

export const collections = { experience, companies, education, providers, newsletter, blog, projects, guestlog };