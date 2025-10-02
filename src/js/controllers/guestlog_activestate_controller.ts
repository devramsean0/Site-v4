import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
    static override values = {
        id: String
    }

    async process(event: any) {
        let path = `/admin/guestlog/${this.idValue}/activestate`
        console.log("Fetching", path);
        await fetch(path, {
            method: "put"
        })
        location.reload()
    }


    declare idValue: string
    declare readonly hasIdValue: boolean
}