import type { JSX } from "react";

import MoreHorizontalIcon from "@/components/icons/more-horizontal";
import { Button } from "@/components/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { ScrollArea } from "@/components/ui/scroll-area"

export type TableCardProps = {
    columns: Array<string>,
    rows: Array<Array<string>>,
    action: JSX.Element,
}

export function TableCard(props: TableCardProps) {
    return (
        <div className="flex flex-1 min-h-0 border border-border rounded-md w-full bg-background/20 shadow-md">
            <ScrollArea className="flex-1 w-full px-4 py-2">
                <Table className="border-b">
                    <TableHeader>
                        <TableRow>
                            {
                                props.columns.map((col =>
                                (
                                    <TableHead className="white font-bold">{col}</TableHead>
                                )
                                ))
                            }
                            <TableHead className="white font-bold text-right">Actions</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {
                            props.rows.map((row => {
                                return (
                                    <TableRow>
                                        {
                                            row.map((rowEntry) =>
                                                <TableCell className="font-medium">{rowEntry}</TableCell>
                                            )
                                        }
                                        {props.action}
                                        {/* <TableCell className="text-right"> */}
                                        {/*     <DropdownMenu> */}
                                        {/*         <DropdownMenuTrigger asChild> */}
                                        {/*             <Button variant="ghost" size="icon" className="size-8"> */}
                                        {/*                 <MoreHorizontalIcon /> */}
                                        {/*                 <span className="sr-only">Open menu</span> */}
                                        {/*             </Button> */}
                                        {/*         </DropdownMenuTrigger> */}
                                        {/*         <DropdownMenuContent align="end"> */}
                                        {/*             <DropdownMenuItem>Edit</DropdownMenuItem> */}
                                        {/*             <DropdownMenuItem>Disable</DropdownMenuItem> */}
                                        {/*             <DropdownMenuSeparator /> */}
                                        {/*             <DropdownMenuItem variant="destructive"> */}
                                        {/*                 Remove */}
                                        {/*             </DropdownMenuItem> */}
                                        {/*         </DropdownMenuContent> */}
                                        {/*     </DropdownMenu> */}
                                        {/* </TableCell> */}
                                    </TableRow>
                                )
                            }))
                        }
                    </TableBody>
                </Table>
            </ScrollArea>
        </div>
    )
}
