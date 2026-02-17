import { AdminSidebar } from "@/components/admin-sidebar";
import { Button } from "@/components/ui/button";
import { Card, CardAction, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { SidebarProvider } from "@/components/ui/sidebar";

export function Admin() {
    return (
        <div className="flex flex-row w-full gap-4 p-4">
            <Card className="w-1/2">
                <CardHeader>
                    <CardTitle>Teams</CardTitle>
                    <CardDescription>
                        Create, edit, and delete teams
                    </CardDescription>

                    <CardAction>
                        <Button className="hover:cursor-pointer">
                            Create Team
                        </Button>
                    </CardAction>
                </CardHeader>

                <CardContent className="flex grow justify-center items-center">
                    <div className="text-muted-foreground">
                        it doesn't look like you have created any teams...
                    </div>
                </CardContent>
            </Card>

            <Card className="w-1/2">
                <CardHeader>
                    <CardTitle>Services</CardTitle>
                    <CardDescription>
                        A service is a named configured check, alongside a configuration and weight
                    </CardDescription>

                    <CardAction>
                        <Button className="hover:cursor-pointer">
                            Create Service
                        </Button>
                    </CardAction>
                </CardHeader>
                <CardContent className="flex grow justify-center items-center">
                    <div className="text-muted-foreground">
                        it doesn't look like you have created any services...
                    </div>
                </CardContent>
            </Card>
        </div>
    )
}
