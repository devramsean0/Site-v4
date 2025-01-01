import { createEffect, createSignal, For } from "solid-js";

export function Experience(props: {positions: any, companies: any}) {
    const [loading, setLoading] = createSignal(true);
    const [completePositions, setCompletePositions] = createSignal<any[]>([]);
    createEffect(() => {
        const updatedPositions = [];
        for (let i = 0; i < props.positions.length; i++) {
            const position = props.positions[i];
            const company = props.companies.find((company: any) => company.id === position.data.company[0]);
            if (company) {
                updatedPositions.push({
                    ...position,
                    data: {
                        ...position.data,
                        company: company
                    }
                });
            }
        }
        setCompletePositions(updatedPositions.sort((a, b) => b.data.start_date - a.data.start_date));
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
                <For each={completePositions()}>{(position: any) =>
                    <div class="w-full pl-4 py-2">
                        <div class="flex w-full">
                            <img class="w-10 h-10 md:w-15 md:h-15 lg:w-20 lg:h-20 mt-6" src={position.data.company.data.logo_link} alt={`${position.data.company.data.name}'s Logo`} loading="lazy"/>
                            <div class="pl-4 w-full">
                                <div class="flex items-center justify-between">
                                    <div>
                                        <h3 class="text-lg font-semibold">{position.data.title}</h3>
                                        <h4 class="text-sm font-semibold">{position.data.type}</h4>
                                        <h4 class="text-sm"><a href={position.data.company.data.link}>{position.data.company.data.name}</a></h4>
                                    </div>
                                    <div class="text-sm text-gray-500">
                                        {`${position.data.start_date.toLocaleString('default', { month: 'long'})} ${position.data.start_date.getFullYear()}`}
                                        - 
                                        {position.data.end_date ? 
                                            `${position.data.end_date.toLocaleString('default', { month: 'long'})} ${position.data.end_date.getFullYear()}`
                                        : `Present`}
                                    </div>
                                </div>
                                <p class="invisible md:visible lg:visible">{position.data.description}</p>
                            </div>
                        </div>
                    </div>
                }
                </For>
            } 
        </>
    )
}