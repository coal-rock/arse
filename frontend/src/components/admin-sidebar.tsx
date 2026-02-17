import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarHeader,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarSeparator,
} from "@/components/ui/sidebar"
import MoreHorizontalIcon from "./icons/more-horizontal"
import { Link } from "react-router"
import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger, DropdownMenuTrigger } from "./ui/dropdown-menu"
import { Button } from "./ui/button"

export function AdminSidebar() {
    return (
        <Sidebar variant="sidebar" collapsible="none" className="border-r border-r-sidebar-border bg-background">
            <SidebarHeader className="flex flex-col items-center">
                <small className="text-foreground/75 whitespace-pre">ARSE Admin Panel</small>
                <SidebarSeparator />
            </SidebarHeader>

            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>
                        Configure
                    </SidebarGroupLabel>

                    <SidebarGroupContent>
                        {
                            [
                                ["/admin/configure/teamService", "Teams / Services"],
                                ["/admin/configure/scoring", "Scoring"],
                            ].map(([link, name]) => (
                                <Link to={link}>
                                    <SidebarMenuButton size="sm">
                                        {name}
                                    </SidebarMenuButton>
                                </Link>
                            ))
                        }
                    </SidebarGroupContent>
                </SidebarGroup>

                <SidebarGroup>
                    <SidebarGroupLabel>
                        Manage
                    </SidebarGroupLabel>

                    <SidebarGroupContent>
                        {
                            [
                                ["/admin/manage/users", "Users"],
                                ["/admin/manage/system", "System"],
                            ].map(([link, name]) => (
                                <Link to={link}>
                                    <SidebarMenuButton size="sm">
                                        {name}
                                    </SidebarMenuButton>
                                </Link>
                            ))
                        }
                    </SidebarGroupContent>
                </SidebarGroup>

            </SidebarContent>


            <SidebarFooter className="flex flex-col items-center">
                <SidebarSeparator />
                <div className="flex flex-row justify-between w-full">
                    <span>
                        <small className="text-muted-foreground whitespace-pre">Logged in as </small>
                        <small className="text-foreground/75">
                            fortnite
                        </small>
                    </span>
                    <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                            <Button variant="outline" size="icon" aria-label="More Options" className="rotate-90">
                                <MoreHorizontalIcon />
                            </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align="end" className="w-40">
                            <DropdownMenuGroup>
                                <DropdownMenuItem>
                                    Pause Scoring
                                </DropdownMenuItem>
                                <DropdownMenuItem>
                                    Reset All Scoring
                                </DropdownMenuItem>
                                <DropdownMenuItem>
                                    Export Scoring (csv)
                                </DropdownMenuItem>
                            </DropdownMenuGroup>
                            <DropdownMenuSeparator />
                            <DropdownMenuGroup>
                                <DropdownMenuItem variant="destructive">
                                    Logout
                                </DropdownMenuItem>
                            </DropdownMenuGroup>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </div>
            </SidebarFooter>
        </Sidebar >
    )
}
