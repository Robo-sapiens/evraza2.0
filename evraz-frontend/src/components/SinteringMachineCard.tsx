import ExhausterItem, { ExhausterProps } from "./ExhausterItem";

export interface SinteringMachineCardProps {
    
    name: string;

    exhausters: ExhausterProps[];
}

const SinteringMachineCard = (props: SinteringMachineCardProps) => {
    return (
      <div className="bg-white">
          <div className="bg-stone-100 p-2 rounded">
              {props.name}
          </div>
          <div className="flex gap-2 pt-2 items-start">
                {props.exhausters.map((exhauster, index) => (
                    <ExhausterItem key={index} {...exhauster} />
                ))}
          </div>
      </div>
    )
}


export default SinteringMachineCard
