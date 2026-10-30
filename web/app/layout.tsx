import type { Metadata } from "next";
import Link from "next/link";
import "./globals.css";

export const metadata: Metadata = {
  title: "Boxwren — sandboxed execution for AI agents",
  description:
    "Local-first WebAssembly sandbox for untrusted agent code. Deny-by-default limits, capability grants, a kill switch, and a stable execution report.",
};

const NAV = [
  { href: "/#install", label: "Install" },
  { href: "/#features", label: "Features" },
  { href: "/#docs", label: "Docs" },
  { href: "/dashboard", label: "Dashboard" },
];

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>
        <header className="site-header">
          <Link href="/" className="brand">
            Boxwren
          </Link>
          <nav>
            {NAV.map((item) => (
              <a key={item.href} href={item.href}>
                {item.label}
              </a>
            ))}
          </nav>
        </header>
        <main>{children}</main>
        <footer className="site-footer">
          <span>MIT OR Apache-2.0</span>
          <a href="https://github.com/rejisterjack/boxwren">GitHub</a>
          <a href="https://crates.io/crates/boxwren">crates.io</a>
        </footer>
      </body>
    </html>
  );
}
