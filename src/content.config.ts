import { defineCollection, z } from 'astro:content';
import { glob, file } from 'astro/loaders';

const experience_position = defineCollection({
    loader: file("src/data/experience_position.json"),
    schema: z.object({
        id: z.number(),
        title: z.string(),
        description: z.string(),
        start_date: z.string(),
        end_date: z.string().optional(),
        company: 
    })
})