export function GuestlogForm() {
    return (
        <form class="py-15 grid grid-cols-2 gap-5">
            <div class="place-self-end">
                <div>
                    <label for="name">Name:</label>
                    <br />
                    <input type="text" id="name" name="name" />
                </div>
                <div>
                    <label for="email">Email:</label>
                    <br />
                    <input type="email" id="email" name="email" />
                </div>
            </div>
            <div>
                <div>
                    <label for="message">Message:</label>
                    <br />
                    <textarea id="message" name="message"></textarea>
                </div>
                <button class="bg-white rounded-md px-[4.5rem] py-1 text-black" type="submit">Submit</button>
            </div>
        </form>
    )
}