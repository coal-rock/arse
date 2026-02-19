import { Outlet } from "react-router";
import { Header } from "./header";

export function Layout() {
    return (
        <div className="flex flex-col w-full h-full">
            <Header />
            <main className="flex flex-1 min-h-0 w-full">
                <Outlet />
            </main>
        </div>
    )
}
