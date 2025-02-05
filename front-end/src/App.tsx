import { ProjectRoutes } from "./aplication/routes/ProjectRoutes"
import { Footer } from "./presentation/components/Footer"
import { Header } from "./presentation/components/Header"


export const App = () => {
  return (
    <>
      <Header />

      <ProjectRoutes />

      <Footer />
    </>
  )
}