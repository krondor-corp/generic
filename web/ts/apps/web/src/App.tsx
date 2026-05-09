import { ThemeProvider } from "./providers/theme/provider";
import Header from "./components/header";
import Footer from "./components/footer";
import Hero from "./components/hero";
import About from "./components/about";
import Features from "./components/features";

function App() {
  return (
    <ThemeProvider defaultTheme="light" storageKey="vite-ui-theme">
      <div className="flex flex-col min-h-screen">
        <Header />
        <main className="flex-grow">
          <Hero />
          <Features />
          <About />
        </main>
        <Footer />
      </div>
    </ThemeProvider>
  );
}

export default App;
