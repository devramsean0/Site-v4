import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
    static override values = {
        path: String,
        id: String
    }

    async process(event: any) {
        let path = `${this.pathValue}/${this.idValue}`
        console.log("Fetching", path);
        await fetch(path, {
            method: "delete"
        })
        location.reload()
    }


    declare pathValue: string
    declare readonly hasPathValue: boolean
    declare idValue: string
    declare readonly hasIdValue: boolean
}