import React, { PropsWithChildren } from 'react'

interface ExhausterStatCardProps {
    number: number;
    stats: Array<{
        name: string;
        value: string;
    }>;
}

const ExhausterStatCard = ({number, stats}: ExhausterStatCardProps) => {
  return (
    <div className="bg-gray-700 text-white p-4 rounded-lg">
        <div className="font-bold border rounded text-center mb-2">
            {number} ПС
        </div>
        <div className="flex flex-col gap-2">
            {
            stats.map((stat) => (
                <div className="flex justify-between gap-8">
                    <div>{stat.name}</div>
                    <div>{stat.value}</div>
                </div>
            ))
            }
        </div>
    </div>
  )
}

export default ExhausterStatCard