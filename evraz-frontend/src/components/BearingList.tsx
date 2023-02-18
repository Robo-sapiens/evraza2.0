import { useState } from "react";
import ExhausterIndicator from "./ExhausterIndicator";
import UpArrow from "./UI/UpArrow";

// Пропсы компонента Bearing:
export interface BearingProps {
    // Номер подшипника
    number: number;
    
    indicators: {
        temperature?: "normal" | "warning" | "danger";
        vibration?: "normal" | "warning" | "danger";
    }
}

const Bearing = (props: BearingProps) => {
    return (
        <div className="flex justify-between">
            <div className="">
                №{props.number} п-к
            </div>
            <div className="flex gap-1"> 
                {
                    props.indicators.temperature && 
                    <ExhausterIndicator indicator={props.indicators.temperature} name="T" />
                }

                {
                    props.indicators.vibration && 
                    <ExhausterIndicator indicator={props.indicators.vibration} name="V" />
                }
            </div>
        </div>
    )
}

export default function BearingList({bearings, name}: {bearings: BearingProps[], name:string}) {
    const [showBearings, setShowBearings] = useState(false)

    return (
        <div className="">
            <button 
                onClick={() => setShowBearings(!showBearings)}
                className="flex gap-2 items-center"
            >
                <div className="bg-stone-100 p-1 rounded">
                    <UpArrow
                        classname="w-4 h-4"
                        style={{transform: showBearings ? "rotate(0deg)" : "rotate(180deg)"}}
                    />
                </div>
                <div>{name}</div>
            </button>
            {showBearings && (
                <div className="flex flex-col gap-2 p-1">
                    {bearings.map((bearing, index) => (
                        <Bearing key={index} {...bearing} />
                    ))}
                </div>
            )}
        </div>
    )
}
