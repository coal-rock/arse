import type { ReactNode } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "./ui/card"

type ChartCardProps = {
    title: String,
    subtitle: String
    children: ReactNode
}

export default function ChartCard(props: ChartCardProps) {
    return (
        <Card className="grow">
            <CardHeader>
                <CardTitle>Current Scores</CardTitle>
                <CardDescription>Last Update: 10:54am</CardDescription>
            </CardHeader>
            <CardContent>
                {props.children}
            </CardContent>
        </Card >
    )
}
