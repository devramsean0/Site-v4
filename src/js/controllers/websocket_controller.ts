import { Controller } from "@hotwired/stimulus"

export default class extends Controller {
    static override values = {
        channel: String
    }

    override connect() {
        const channel = this.channelValue
        this.ws = new WebSocket(`ws://${location.host}/ws/${channel}`)

        this.ws.onmessage = (event) => {
            console.log(`${channel}: Recieved ${event.data}`)
            // Dispatch a custom event with the payload
            document.dispatchEvent(new CustomEvent("wsmessage", {
                detail: { channel, data: event.data }
            }))
        }

        this.ws.onopen = () => {
            console.log(`Connected to ${channel}`)
        }

        this.ws.onclose = () => {
            console.log(`Disconnected from ${channel}`)
        }
    }

    override disconnect() {
        if (this.ws) {
            this.ws.close()
        }
    }

    send(event: any) {
        // Send a message (e.g. from a form submit)
        const data = event.detail || {}
        this.ws.send(JSON.stringify(data))
    }

    declare ws: WebSocket;
    declare channelValue: string;
    declare hasChanelValue: boolean;
}
