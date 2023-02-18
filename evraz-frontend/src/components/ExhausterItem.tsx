import { useMemo } from "react"
import BearingList, { BearingProps } from "./BearingList"
import RightButton from "./UI/RightButton"
import cring from "../assets/cring.png"
import ExhausterIndicator from "./ExhausterIndicator"
import { useNavigate } from "react-router"

const ExhausterItem = (props: ExhausterProps) => {
    const navigate = useNavigate()

    const warnedBearings = useMemo(() => {
        return props.bearings.filter(bearing => {
            return bearing.indicators.temperature === "danger" || bearing.indicators.vibration === "danger"
            || bearing.indicators.temperature === "warning" || bearing.indicators.vibration === "warning"
        })
    }, [props.bearings])

    const normalBearings = useMemo(() => {
        return props.bearings.filter(bearing => {
            return bearing.indicators.temperature !== "danger" && bearing.indicators.vibration !== "danger"
            && bearing.indicators.temperature !== "warning" && bearing.indicators.vibration !== "warning"
        })
    }, [props.bearings])
    
    return (
        <div className="border border-stone-300 rounded-md min-w-[300px]">
            <div className="bg-stone-600 text-white p-2 flex justify-between gap-1
            border-b border-gray-300 rounded-t-sm
            ">
                <div className="flex gap-2">
                    <div className={`w-4 h-4 rounded-full mt-1 border-2 border-white
                     ${props.status === "working" ? "bg-green-500" : "bg-red-500"}`
                    }>
                    </div>
                    <div className="">
                        Эксгаустер {props.name}
                    </div>
                </div>
                
                <button 
                    className="w-6 h-6 bg-stone-100 text-stone-500 rounded p-1 border border-stone-200"
                    onClick={() => navigate(`exhauser/${props.name}/monitor`)}
                >
                    <RightButton classname="w-4 h-4" strokeWidth={1.5} stroke="currentColor" />
                </button>
            </div>
            <div className="p-4 flex flex-col gap-2">
                <div className="">
                    <div className="flex gap-2 justify-between">
                        <div className="font-bold">
                            Ротор №{props.rotorNumber}
                        </div>
                        <div className="bg-stone-100 py-1 px-2 rounded text-sm">
                            {props.rotorChangeDate.toLocaleDateString()}
                        </div>
                        <button className="rounded-sm bg-stone-100 px-1 border border-stone-300">
                            Изменить
                        </button>
                    </div>
                    <hr className="my-2" />
                    <div className="text-sm flex flex-col gap-2">
                        <div className="font-bold">Последняя замена ротора</div>
                        <div className="bg-stone-50 p-2 rounded flex items-center gap-2">
                            <div className="bg-stone-100 py-1 px-2 rounded font-bold text-base">
                                {props.rotorWorkingTime} сут
                            </div>
                            <div className="">
                                <div>
                                    Прогноз
                                </div>
                                <div className="font-bold">
                                    {props.rotorWorkingTimeForecast} сут
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <img src={cring} alt="cring" className="m-auto" />

                <div className="flex flex-col gap-2">
                    <div>
                    {
                    warnedBearings.length > 0 && 
                    <BearingList name="Предупреждения" bearings={warnedBearings} />
                    }
                    </div>
                    <div>
                    {
                    normalBearings.length > 0 && 
                    <BearingList name="Подшипники" bearings={normalBearings} />
                    }
                    </div>
                </div>

                <div className="flex flex-col gap-2 p-1">
                    <div className="flex justify-between">
                        <div>Уровень масла</div>
                        <ExhausterIndicator indicator="normal" name="L" /> 
                    </div>
                    <div className="flex justify-between">
                        <div>Давление масла</div>
                        <ExhausterIndicator indicator="warning" name="P" />
                    </div>
                </div>
            </div>
        </div>
    )
}

// Пропсы компонента Exhauster:
export interface ExhausterProps {
    // Статус работы
    status: 'working' | 'not working';

    // Название эксгаустера
    name: string;

    // Номер ротора
    rotorNumber: number;

    // Дата замены ротора
    rotorChangeDate: Date;

    // Время работы ротора
    rotorWorkingTime: number;

    // Прогнозное время работы до отказа
    rotorWorkingTimeForecast: number;

    // Подшипники
    bearings: BearingProps[];

    // Уровень масла
    oilLevel: 'normal' | 'warning' | 'danger';

    // Давление масла
    oilPressure: 'normal' | 'warning' | 'danger';
}

export default ExhausterItem