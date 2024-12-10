import "dotenv/config";
import fs from "fs";

const TABLES = [
    {
        file: "guestlog",
        table: "tblGMCzu5zQoRWVPz",
        view: "active",
    },
    {
        file: "experience",
        table: "tblosHvcM8CSeKN2d",
        view: "active",
    },
    {
        file: "companies",
        table: "tbljgpSDbSFQQIV6H",
        view: "all",
    },
    {
        file: "education",
        table: "tbllJIOc7KzTnc0cq",
        view: "active",
    },
    {
        file: "providers",
        table: "tbla5ycwilNnAMAsV",
        view: "all",
    },
    {
        file: "newsletter",
        table: "tblbsE641Aiifk4JT",
        view: "sent",
    },
    {
        file: "blog",
        table: "tblZ6yDcCp53lnDfz",
        view: "active",
    },
    {
        file: "comments",
        table: "tblN1q82TgvB2K2ym",
        view: "active",
    },
    {
        file: "projects",
        table: "tblqwjiEDN4BX3DtC",
        view: "active",
    }
]

fs.mkdirSync("src/data", { recursive: true });
TABLES.forEach(async ({ file, table, view, fields }) => {
    const res = await fetch(`https://api.airtable.com/v0/appjAFKlWvVpwaM7K/${table}?view=${view}`, {
        headers: {
            Authorization: `Bearer ${process.env.AIRTABLE_API_KEY}`
        }
    });
    const data = await res.json();
    let json = JSON.stringify(data.records);
    fs.writeFileSync(`src/data/${file}.json`, json);
    console.log(`Fetched ${file}.json`);
});