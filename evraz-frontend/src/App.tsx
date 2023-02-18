import { Route, Routes } from "react-router";
import ExhausterStatCard from "./components/ExhausterStatCard";
import SinteringMachineCard from "./components/SinteringMachineCard";
import ExhausterScreen from "./pages/ExhausterScreen";
import MainScreen from "./pages/MainScreen";
import TrendScreen from "./pages/TrendScreen";
import { testExhausters, testSinteringMachines } from "./testProps";

export default function App() {

  return (
    <div>
        <Routes>
            <Route path="/" element={<MainScreen sinteringMachines={testSinteringMachines}/>} />
            <Route path="/exhauser/:id/monitor" element={<ExhausterScreen />} />
            <Route path="/exhauser/:id/trends" element={<TrendScreen />} />

            <Route path="/test" element={<div className="flex flex-col min-h-screen justify-center items-center">
              <ExhausterStatCard 
              number={1}
              stats={[
                  {
                      name: "T, °C",
                      value: "100"
                  },
                  {
                      name: "В, мм/с",
                      value: "0.26",
                  },
                ]}
              />
            </div>} />
        </Routes>
    </div>
  )
}
