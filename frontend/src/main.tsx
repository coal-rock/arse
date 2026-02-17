import { StrictMode } from "react"
import { createRoot } from "react-dom/client"

import "./index.css"
import { BrowserRouter, Route, Routes } from "react-router"
import { Scoreboard } from "./pages/scoreboard.tsx"
import { NotFound } from "./pages/not-found.tsx"
import { Admin } from "./pages/admin.tsx"
import { AdminLayout } from "./components/admin-layout.tsx"
import { Layout } from "./components/layout.tsx"

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route index element={<Scoreboard />} />
          <Route path="*" element={<NotFound />} />
          <Route path="/admin" element={<AdminLayout />}>
            <Route index element={<Admin />} />
          </Route>
        </Route>
      </Routes>
    </BrowserRouter>
  </StrictMode >
)
