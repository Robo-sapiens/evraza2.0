import { SinteringMachineCardProps } from "./components/SinteringMachineCard";
import { ExhausterProps } from "./components/ExhausterItem";

export const testExhausters : ExhausterProps[] = [
    {
        name: "У-171",
        status: "working",
        rotorNumber: 1,
        rotorChangeDate: new Date(),
        rotorWorkingTime: 5,
        rotorWorkingTimeForecast: 10,
        oilLevel: "normal",
        oilPressure: "normal",
        bearings: [
            {
                number: 1,
                indicators: {
                    temperature: 'normal',
                    vibration: 'normal'
                }
            },
            {
                number: 2,
                indicators: {
                    vibration: 'normal'
                }
            },
            {
                number: 3,
                indicators: {
                    temperature: 'normal'
                }
            },
            {
                number: 4,
                indicators: {
                    temperature: 'warning',
                    vibration: 'normal'
                }
            },
          ]
    },
    {
      name: "У-172",
      status: "working",
      rotorNumber: 1,
      rotorChangeDate: new Date(),
      rotorWorkingTime: 5,
      rotorWorkingTimeForecast: 10,
      oilLevel: "warning",
      oilPressure: "normal",
      bearings: [
            {
                number: 1,
                indicators: {
                    temperature: 'normal',
                    vibration: 'normal'
                }
            },
            {
                number: 2,
                indicators: {
                    temperature: 'warning',
                    vibration: 'normal'
                }
            },
            {
                number: 3,
                indicators: {
                    vibration: 'normal'
                }
            },
            {
                number: 4,
                indicators: {
                    temperature: 'danger',
                }
            },
            {
                number: 5,
                indicators: {
                    temperature: 'normal',
                    vibration: 'danger'
                }
            },
        ]
  },
  ];

  export const testSinteringMachines : SinteringMachineCardProps[] = [
    {
        name: "Агломашина 1",
        exhausters: testExhausters
    },
    {
        name: "Агломашина 2",
        exhausters: testExhausters
    },
  ]