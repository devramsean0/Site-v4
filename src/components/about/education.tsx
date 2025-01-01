import { createSignal, createEffect, For } from "solid-js";

export function Education(props: {education: any, providers: any}) {
    const [loading, setLoading] = createSignal(true);
    const [completeEducation, setCompleteEducation] = createSignal<any[]>([]);
    createEffect(() => {
        const updatedEducation = [];
        for (let i = 0; i < props.education.length; i++) {
            const position = props.education[i];
            const provider = props.providers.find((provider: any) => provider.id === position.data.provider[0]);
            if (provider) {
                updatedEducation.push({
                    ...position,
                    data: {
                        ...position.data,
                        provider: provider
                    }
                });
            }
        }
        setCompleteEducation(updatedEducation.sort((a, b) => b.data.start_date - a.data.start_date));
        setLoading(false);
    });
    return (
        <>
            {loading()
                ?
                    <div class="w-full pl-4 py-2 flex justify-center">
                        <h3 class="text-3xl">Loading experience</h3>
                    </div>
                :
                <For each={completeEducation()}>{(education: any) =>
                    <div class="w-full pr-4 py-2">
                        <div class="flex w-full">
                            <img class="h-10 w-fit md:h-15 lg:h-20 mt-6" src={education.data.provider.data.logo_link} alt={`${education.data.provider.data.name}'s Logo`} loading="lazy"/>
                            <div class="pl-4 w-full">
                                <div class="flex items-center justify-between">
                                    <div>
                                        <h3 class="text-lg font-semibold">{education.data.title}</h3>
                                        <h4 class="text-sm"><a href={education.data.provider.data.link}>{education.data.provider.data.name}</a></h4>
                                    </div>
                                    <div class="text-sm">
                                        {`${education.data.start_date.toLocaleString('default', { month: 'long'})} ${education.data.start_date.getFullYear()}`}
                                        - 
                                        {education.data.end_date ? 
                                            `${education.data.end_date.toLocaleString('default', { month: 'long'})} ${education.data.end_date.getFullYear()}`
                                        : `Present`}
                                    </div>
                                </div>
                                <p class="invisible md:visible lg:visible">{education.data.description}</p>
                            </div>
                        </div>
                    </div>
                }
                </For>
            }
        </>
    )
}