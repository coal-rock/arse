export function Overview() {
    return (
        <div className="flex flex-row justify-evenly w-full gap-4 p-4 overflow-scroll ">
            <CurrentScoreBarChart />
            <HistoricScoreLineChart />
        </div>
    )
}

