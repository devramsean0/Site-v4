export function StaticCollectionLookupF(collection: any[], lookupValue: string, lookupKey: string = "id", resultKey: string) {
    return collection.find((data: any) => data[lookupKey] === lookupValue).data[resultKey]
}