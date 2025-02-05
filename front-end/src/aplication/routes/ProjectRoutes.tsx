
import { BrowserRouter, Routes, Route } from "react-router";
import { Home } from "../../presentation/pages/Home"


export const ProjectRoutes = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
      </Routes>
    </BrowserRouter>
  );
};