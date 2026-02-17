
import { Link } from "react-router"

import {
    NavigationMenu,
    NavigationMenuItem,
    NavigationMenuLink,
    NavigationMenuList,
    navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu"

export function Header() {
    return (
        <div className="shrink-0 w-full border-b border-b-sidebar-accent-foreground/10 items-end">
            <NavigationMenu className="p-1 justify-between min-w-full">
                <NavigationMenuList className="gap-0.5">
                    {
                        [
                            ["/", "Scoreboard"],
                            ["/overview", "Service Overview"],
                        ].map(([link, name]) => (
                            <NavigationMenuItem>
                                <NavigationMenuLink asChild className={navigationMenuTriggerStyle()}>
                                    <Link to={link}>{name}</Link>
                                </NavigationMenuLink>
                            </NavigationMenuItem>
                        ))
                    }
                </NavigationMenuList>

                <NavigationMenuList className="gap-0.5">
                    {
                        [
                            ["/admin", "Admin"]
                        ].map(([link, name]) => (
                            <NavigationMenuItem>
                                <NavigationMenuLink asChild className={navigationMenuTriggerStyle()}>
                                    <Link to={link}>{name}</Link>
                                </NavigationMenuLink>
                            </NavigationMenuItem>
                        ))
                    }
                </NavigationMenuList>
            </NavigationMenu>
        </div>
    )
}
