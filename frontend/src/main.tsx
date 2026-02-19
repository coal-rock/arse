import { StrictMode } from "react"
import { createRoot } from "react-dom/client"

import "./index.css"
import { BrowserRouter, Route, Routes } from "react-router"
import { Scoreboard } from "./pages/scoreboard.tsx"
import { NotFound } from "./pages/not-found.tsx"
import { ConfigureTeamService } from "./pages/configure-team-service.tsx"
import { AdminLayout } from "./components/admin-layout.tsx"
import { Layout } from "./components/layout.tsx"
import { ConfigureScoring } from "./pages/configure-scoring.tsx"
import { ManageSystem } from "./pages/manage-system.tsx"
import { ManageUsers } from "./pages/manage-users.tsx"

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route index element={<Scoreboard />} />
          <Route path="*" element={<NotFound />} />
          <Route path="admin" element={<AdminLayout />}>
            <Route index element={<ConfigureTeamService />} />
            <Route path="configure/teamService" element={<ConfigureTeamService />} />
            <Route path="configure/scoring" element={<ConfigureScoring />} />
            <Route path="manage/system" element={<ManageSystem />} />
            <Route path="manage/users" element={<ManageUsers />} />
          </Route>
        </Route>
      </Routes>
    </BrowserRouter>
  </StrictMode >
)
