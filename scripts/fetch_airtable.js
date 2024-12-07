import "dotenv/config";
import fs from "fs";

const TABLES = [
    {
        file: "experience",
        table: "tblosHvcM8CSeKN2d",
        view: "active",
        fields: [
            "id",
            "title",
            "description",
            "start_date",
            "end_date",
            "company",
            "type"
        ],
    },
    {
        file: "companies",
        table: "tbljgpSDbSFQQIV6H",
        view: "all",
        fields: [
            "name",
            "logo_link",
            "link"
        ]
    },
    {
        file: "education",
        table: "tbllJIOc7KzTnc0cq",
        view: "active",
        fields: [
            "id",
            "title",
            "description",
            "start_date",
            "end_date",
            "provider",
        ],
    },
    {
        file: "providers",
        table: "tbla5ycwilNnAMAsV",
        view: "all",
        fields: [
            "name",
            "logo_link",
            "link"
        ]
    },
    {
        file: "newsletter",
        table: "tblbsE641Aiifk4JT",
        view: "sent",
        fields: [
            "id",
            "email",
            "contents"
        ]
    },
    {
        file: "blog",
        table: "tblZ6yDcCp53lnDfz",
        view: "active",
        fields: [
            "id",
            "title",
            "description",
            "tags",
            "comments",
            "comments_enabled",
        ],
    },
    {
        file: "comments",
        table: "tblN1q82TgvB2K2ym",
        view: "active",
        fields: [
            "id",
            "email",
            "comment"
        ]
    }
]

TABLES.forEach(async ({ file, table, view, fields, unfurl_records = [] }) => {
    const res = await fetch(`https://api.airtable.com/v0/appjAFKlWvVpwaM7K/${table}?view=${view}&${encodeURIComponent(fields)}`, {
        headers: {
            Authorization: `Bearer ${process.env.AIRTABLE_API_KEY}`
        }
    });
    const data = await res.json();
    let json = JSON.stringify(data.records);
    fs.writeFileSync(`src/data/${file}.json`, json);
    console.log(`Fetched ${file}.json`);
});