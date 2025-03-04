import type { Loader, LoaderContext } from 'astro/loaders';
import { z } from 'astro:content';
import { S3Client, ListObjectsV2Command, paginateListObjectsV2, GetObjectCommand } from "@aws-sdk/client-s3";

export function s3Loader(options: {endpoint: string, bucket: string, auth: {key_id: string, access_key: string}}): Loader {
    return {
        name: "S3-loader",
        load: async (context: LoaderContext): Promise<void> => {
            const s3Client = new S3Client({
                region: "us-east-1",
                endpoint: options.endpoint,
                credentials: {
                    accessKeyId: options.auth.key_id,
                    secretAccessKey: options.auth.access_key
                }
            })
            const paginator = paginateListObjectsV2(
                { client: s3Client },
                { Bucket: options.bucket },
              );
              for await (const page of paginator) {
                const objects = page.Contents;
                if (objects) {
                  // For every object in each page, delete it.
                  for (const object of objects) {
                    const data = await s3Client.send(
                      new GetObjectCommand({
                        Bucket: options.bucket,
                        Key: object.Key,
                      })
                    );
                    if (!object.Key) return context.logger.error(`No key found for ${object}`);
                    if (!data.Body) return context.logger.error(`No body found for ${object.Key}`);
                    context.store.set({ id: object.Key, data: await context.parseData({id: object.Key, data: { data: data.Body}}) });
                  }
                }
              }
        }
    }
}