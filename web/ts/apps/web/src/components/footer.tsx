import { Facebook, Twitter, Instagram, Github, Mail } from "lucide-react";

export default function Footer() {
  return (
    <footer className="bg-foreground text-background w-full">
      <div className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div>
            <h3 className="text-lg font-semibold mb-4">generic</h3>
            <p className="text-sm opacity-90">
              A modern, full-stack TypeScript application template with
              authentication, API integration, and a clean UI.
            </p>
          </div>
          <div>
            <h3 className="text-lg font-semibold mb-4">Quick Links</h3>
            <ul className="space-y-2">
              <li>
                <a
                  href="#features"
                  className="text-sm opacity-90 hover:opacity-100 transition-opacity"
                >
                  Features
                </a>
              </li>
              <li>
                <a
                  href="/app/dashboard"
                  className="text-sm opacity-90 hover:opacity-100 transition-opacity"
                >
                  Dashboard
                </a>
              </li>
              <li>
                <a
                  href="/app/login"
                  className="text-sm opacity-90 hover:opacity-100 transition-opacity"
                >
                  Login
                </a>
              </li>
            </ul>
          </div>
          <div>
            <h3 className="text-lg font-semibold mb-4">Connect With Us</h3>
            <div className="flex space-x-4">
              <a
                href="#"
                className="opacity-90 hover:opacity-100 transition-opacity"
              >
                <Facebook size={24} />
              </a>
              <a
                href="#"
                className="opacity-90 hover:opacity-100 transition-opacity"
              >
                <Twitter size={24} />
              </a>
              <a
                href="#"
                className="opacity-90 hover:opacity-100 transition-opacity"
              >
                <Instagram size={24} />
              </a>
              <a
                href="#"
                className="opacity-90 hover:opacity-100 transition-opacity"
              >
                <Github size={24} />
              </a>
              <a
                href="mailto:contact@example.com"
                className="opacity-90 hover:opacity-100 transition-opacity"
              >
                <Mail size={24} />
              </a>
            </div>
          </div>
        </div>
        <div className="mt-8 pt-8 border-t border-background/10 text-center text-sm opacity-75">
          &copy; {new Date().getFullYear()} generic. All rights reserved.
        </div>
      </div>
    </footer>
  );
}
