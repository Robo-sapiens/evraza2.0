interface ExhausterIndicatorProps {
    indicator: "normal" | "warning" | "danger";
    name: 'V' | 'T' | 'L' | 'P';
}

const colors = {
    normal: "border-stone-200 text-stone-200",
    warning: "border-yellow-400 text-yellow-400",
    danger: "border-red-400 text-red-400"
}

// TODO: add indicators SVGs
const ExhausterIndicator = ({indicator, name}: ExhausterIndicatorProps) => {

    return (
        <div className={
            `py-1 px-2 bg-stone-100 border rounded ${colors[indicator]}`
            }>
            <span className="text-black">{name}</span>
        </div>
    )
}

export default ExhausterIndicator