import MoreHorizontalIcon from "@/components/icons/more-horizontal";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle, } from "@/components/ui/card";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";
import { TableCell, } from "@/components/ui/table";
import { TableCard } from "@/components/table-card";


export function ManageUsers() {
    const users = Array(20)

    users.fill(
        [
            "coal",
            "disabled",
            "April 15th, 2026",
        ],
        0, 100
    )

    return (
        <div className="flex w-full h-full p-4">
            <Card className="flex flex-col w-full h-full">
                <CardHeader>
                    <CardTitle>
                        Users
                    </CardTitle>

                    <CardDescription>
                        Create, delete, and edit users
                    </CardDescription>
                </CardHeader>

                <CardContent className="flex flex-col flex-1 min-h-0 w-full">
                    <TableCard columns={["Name", "Status", "Created"]} rows={users} action={
                        <TableCell className="text-right">
                            <DropdownMenu>
                                <DropdownMenuTrigger asChild>
                                    <Button variant="ghost" size="icon" className="size-8">
                                        <MoreHorizontalIcon />
                                        <span className="sr-only">Open menu</span>
                                    </Button>
                                </DropdownMenuTrigger>
                                <DropdownMenuContent align="end">
                                    <DropdownMenuItem>Edit</DropdownMenuItem>
                                    <DropdownMenuItem>Disable</DropdownMenuItem>
                                    <DropdownMenuSeparator />
                                    <DropdownMenuItem variant="destructive">
                                        Remove
                                    </DropdownMenuItem>
                                </DropdownMenuContent>
                            </DropdownMenu>
                        </TableCell>
                    }>
                    </TableCard>
                </CardContent>
            </Card>
        </div >
    )
}
