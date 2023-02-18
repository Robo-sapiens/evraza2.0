import SinteringMachineCard, { SinteringMachineCardProps } from "../components/SinteringMachineCard"

export interface MainScreenProps {
    sinteringMachines: SinteringMachineCardProps[];
}

export default function MainScreen({sinteringMachines}: MainScreenProps) {
    return (
        <div className="flex flex-col gap-4 p-4">
            <header className="font-bold text-xl bg-stone-50 p-4 rounded-md 
            border border-gray-200
            ">
                Главный экран
            </header>
            <main className="flex gap-4">
                {
                    sinteringMachines.map((sinteringMachine) => (
                        <SinteringMachineCard {...sinteringMachine} />
                    ))
                }
            </main>
        </div>
    )
}