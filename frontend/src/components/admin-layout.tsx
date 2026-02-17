import { Outlet } from "react-router";
import { AdminSidebar } from "./admin-sidebar";
import { SidebarProvider } from "./ui/sidebar";

export function AdminLayout() {
    return (
        <SidebarProvider className="flex grow">
            <AdminSidebar>
            </AdminSidebar>
            <main className="flex grow">
                <Outlet />
            </main>
        </SidebarProvider>
    )
}
