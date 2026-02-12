import { Bar, BarChart, CartesianGrid, XAxis } from "recharts"
import { ChartContainer, ChartTooltip, ChartTooltipContent, type ChartConfig } from "./ui/chart"
import ChartCard from "./chart-card"

export default function CurrentScoreBarChart() {
    const chartData = [
        { team: "Team 1", score: 186 },
        { team: "Team 2", score: 305 },
        { team: "Team 3", score: 237 },
        { team: "Team 4", score: 73 },
        { team: "Team 5", score: 209 },
        { team: "Team 6", score: 214 },
    ]

    const chartConfig = {
        score: {
            label: "Score",
            color: "var(--color-chart-1)",
        },
    } satisfies ChartConfig

    return (
        <ChartCard title="Current Score" subtitle="Last Update: 10:54am">
            <ChartContainer config={chartConfig}>
                <BarChart accessibilityLayer data={chartData}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis
                        dataKey="team"
                        tickLine={false}
                        tickMargin={10}
                        axisLine={false}
                    />
                    <ChartTooltip
                        cursor={false}
                        content={<ChartTooltipContent />}
                    />
                    <Bar dataKey="score" fill="var(--color-chart-2)" radius={4} />
                </BarChart>
            </ChartContainer>
        </ChartCard>

    )
}
