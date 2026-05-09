import { Card } from "@/components/ui/card";
import { Globe, Coins, User, Zap, Package, Rocket } from "lucide-react";

export default function Features() {
  const useCases = [
    {
      icon: <Coins className="h-12 w-12 mx-auto mb-4" />,
      title: "Client-Side Crypto",
      description:
        "Build cryptocurrency applications with TypeScript and modern web APIs.",
      badge: "Crypto",
    },
    {
      icon: <Globe className="h-12 w-12 mx-auto mb-4" />,
      title: "Public Websites",
      description:
        "Create public-facing websites with Vite, React, and Tailwind CSS.",
      badge: "Web",
    },
    {
      icon: <User className="h-12 w-12 mx-auto mb-4" />,
      title: "Portfolios",
      description:
        "Build personal portfolios and showcase projects with a clean, responsive design.",
      badge: "Portfolio",
    },
  ];

  const infrastructure = [
    {
      icon: <Zap className="h-12 w-12 mx-auto mb-4" />,
      title: "Turbo Monorepo",
      description:
        "Cached builds and efficient task management across multiple packages.",
      badge: "Build",
    },
    {
      icon: <Package className="h-12 w-12 mx-auto mb-4" />,
      title: "Docker Ready",
      description:
        "Dockerization patterns and boilerplate for deployment anywhere.",
      badge: "Deploy",
    },
    {
      icon: <Rocket className="h-12 w-12 mx-auto mb-4" />,
      title: "CI/CD Pipeline",
      description: "Basic deployment pipeline for shipping updates quickly.",
      badge: "Ship",
    },
  ];

  return (
    <section id="features" className="py-32 bg-muted">
      <div className="container mx-auto px-4">
        <h2 className="text-4xl font-bold text-center mb-6">Use Cases</h2>
        <p className="text-xl text-center text-muted-foreground mb-16 max-w-2xl mx-auto">
          Build client-side applications with TypeScript and Vite
        </p>
        <div className="grid gap-8 md:grid-cols-3 mb-24">
          {useCases.map((feature, index) => (
            <div key={index} className="text-center">
              <Card className="h-full p-8 relative hover:shadow-lg transition-shadow">
                <span className="absolute top-2 right-2 text-xs bg-muted px-2 py-1 rounded">
                  {feature.badge}
                </span>
                {feature.icon}
                <h3 className="text-xl font-semibold mb-2">{feature.title}</h3>
                <p className="text-muted-foreground">{feature.description}</p>
              </Card>
            </div>
          ))}
        </div>

        <h2 className="text-4xl font-bold text-center mb-6">Infrastructure</h2>
        <p className="text-xl text-center text-muted-foreground mb-16 max-w-2xl mx-auto">
          Built-in tools for development and deployment
        </p>
        <div className="grid gap-8 md:grid-cols-3">
          {infrastructure.map((feature, index) => (
            <div key={index} className="text-center">
              <Card className="h-full p-8 relative hover:shadow-lg transition-shadow">
                <span className="absolute top-2 right-2 text-xs bg-muted px-2 py-1 rounded">
                  {feature.badge}
                </span>
                {feature.icon}
                <h3 className="text-xl font-semibold mb-2">{feature.title}</h3>
                <p className="text-muted-foreground">{feature.description}</p>
              </Card>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
