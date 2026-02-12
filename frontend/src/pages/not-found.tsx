import {
    Empty,
    EmptyContent,
    EmptyDescription,
    EmptyHeader,
    EmptyTitle,
} from "@/components/ui/empty"

export function NotFound() {
    return (
        <Empty>
            <EmptyHeader>
                <EmptyTitle>404 - Not Found</EmptyTitle>
                <EmptyDescription>
                    The page you&apos;re looking for doesn&apos;t exist. If you think that it should,
                    cosider contributing to the GitHub!
                    :)
                </EmptyDescription>
            </EmptyHeader>
            <EmptyContent>
                <EmptyDescription>
                    <a href="https://github.com/coal-rock/arse">github.com/coal-rock/arse</a>
                </EmptyDescription>
            </EmptyContent>
        </Empty>
    )
}
