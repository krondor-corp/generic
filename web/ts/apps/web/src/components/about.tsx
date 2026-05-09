import { Card } from "@/components/ui/card";
import { Calendar, Shield, Database } from "lucide-react";

export default function About() {
  const roadmapItems = [
    {
      icon: <Shield className="h-8 w-8 mb-3" />,
      title: "Public API Deployment",
      description: "Basic public-facing API for accessing protected resources",
      status: "Coming Soon",
    },
    {
      icon: <Calendar className="h-8 w-8 mb-3" />,
      title: "OAuth Integration",
      description: "Optional OAuth authentication for user management",
      status: "Planned",
    },
    {
      icon: <Database className="h-8 w-8 mb-3" />,
      title: "Database Pattern",
      description:
        "Type-safe relational database implementation with PostgreSQL",
      status: "Planned",
    },
  ];

  return (
    <section id="roadmap" className="py-32 bg-background">
      <div className="container mx-auto px-4">
        <h2 className="text-4xl font-bold text-center mb-6">Roadmap</h2>
        <p className="text-xl text-center text-muted-foreground mb-16 max-w-2xl mx-auto">
          Upcoming features and improvements
        </p>
        <div className="grid gap-8 md:grid-cols-3">
          {roadmapItems.map((item, index) => (
            <Card
              key={index}
              className="p-8 text-center relative hover:shadow-lg transition-shadow h-full"
            >
              <span className="absolute top-2 right-2 text-xs bg-muted px-2 py-1 rounded">
                {item.status}
              </span>
              {item.icon}
              <h3 className="text-xl font-semibold mb-2">{item.title}</h3>
              <p className="text-muted-foreground">{item.description}</p>
            </Card>
          ))}
        </div>
      </div>
    </section>
  );
}
