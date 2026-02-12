import { StrictMode } from "react"
import { createRoot } from "react-dom/client"

import "./index.css"
import { BrowserRouter, Route, Routes } from "react-router"
import { Header } from "./components/header.tsx"
// import { Scoreboard } from "./pages/scoreboard.tsx"
import { NotFound } from "./pages/not-found.tsx"
import { Overview } from "./pages/overview.tsx"

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <BrowserRouter>
      <Header />
      <Routes>
        {/* <Route index element={<Scoreboard />} /> */}
        <Route path="/overview" element={<Overview />} />
        {/* <Route path="/admin" element={<Admin />} /> */}
        <Route path="*" element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  </StrictMode >
)
