import { BrowserRouter, Routes, Route } from "react-router";
import { Home } from "../../presentation/pages/Home"
import { Login } from "../../presentation/pages/Login";
import { About } from "../../presentation/pages/About";
import { FAQ } from "../../presentation/pages/FAQ";
import { RegisterBook } from "../../presentation/pages/RegisterBook";
import { RegisterAuthor } from "../../presentation/pages/RegisterAuthor";
import { BookPage } from "../../presentation/pages/BookPage";
import { AuthorPage } from "../../presentation/pages/AuthorPage";


export const ProjectRoutes = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<Login />} />
        <Route path="/sobre" element={<About />} />
        <Route path="/faq" element={<FAQ />} />


        <Route path="/livro/:bookId" element={<BookPage />} />
        <Route path="/livro/cadastro" element={<RegisterBook />} />


        <Route path="/autor/:authorId" element={<AuthorPage />} />
        <Route path="/autor/cadastro" element={<RegisterAuthor />} />
      </Routes>
    </BrowserRouter>
  );
};