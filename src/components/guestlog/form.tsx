export function GuestlogForm() {
    const handleSubmit = async (e: any) => {
        e.preventDefault();
        const form = e.target;
        const formData = new FormData(form);
        const body = Object.fromEntries(formData);
        const res = await fetch('/api/guestlog', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(body),
        });
        if (res.ok) {
            form.reset();
        } else {
            alert('Failed to submit');
        }
    }
    return (
        <form class="flex items-center justify-center flex-col gap-4" onSubmit={handleSubmit}>
            <div class="flex flex-row gap-4">
                <div>
                    <label for="name">Name</label>
                    <br />
                    <input type="text" id="name" name="name" required />
                </div>
                <div>
                    <label for="email">Email</label>
                    <br />
                    <input type="email" id="email" name="email" required />
                </div>
            </div>
            <div>
                <label for="message">Message</label>
                <br />
                <textarea id="message" name="message" required></textarea>
            </div>
            <div>
                <label for="gravatar_enabled">Use Gravatar? </label>
                <input type="checkbox" id="gravatar_enabled" name="gravatar_enabled" class="w-4 h-4 rounded ring-blue-500" />
            </div>
            <button type="submit" class="px-5 py-1 rounded-lg text-black bg-white hover:bg-blue-500">Submit</button>
        </form>
    )
}