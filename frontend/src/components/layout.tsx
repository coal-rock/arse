import { Outlet } from "react-router";
import { Header } from "./header";

export function Layout() {
    return (
        <div className="flex grow flex-col">
            <Header />
            <main className="flex grow">
                <Outlet />
            </main>
        </div>
    )
}
