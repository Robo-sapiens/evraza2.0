import { useParams } from "react-router";

export default function ExhausterScreen() {
    const { id } = useParams<{id: string}>();
    return (
        <div className="flex flex-col gap-4 p-4">
            <header className="font-bold text-xl bg-stone-50 p-4 rounded-md 
            border border-gray-200
            ">
                Эксгаустер {id}
            </header>
            <main className="flex gap-4">
            </main>
        </div>
    )
}