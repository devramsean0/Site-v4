import { Controller } from "@hotwired/stimulus";

export default class extends Controller {
    static override targets = ["output"];
    declare readonly hasOutputTarget: boolean
    declare readonly outputTarget: HTMLParagraphElement

    recieved(event: any) {
        if (event.detail.channel != "project") return
        this.outputTarget.innerHTML = event.detail.data;
    }
}