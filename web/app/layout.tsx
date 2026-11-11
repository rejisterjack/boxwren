import type { Metadata, Viewport } from "next";
import Link from "next/link";
import Navbar from "./components/Navbar";
import BoxwrenLogo from "./components/BoxwrenLogo";
import "./globals.css";

export const viewport: Viewport = {
  themeColor: "#060907",
  colorScheme: "dark",
  width: "device-width",
  initialScale: 1,
};

export const metadata: Metadata = {
  metadataBase: new URL(process.env.NEXT_PUBLIC_APP_URL || "https://boxwren.dev"),
  title: "Boxwren — Sandboxed WebAssembly Execution Runtime for AI Agents",
  description:
    "Contain untrusted AI agent code at 1 µs overhead. In-process Wasmtime isolation, deny-by-default capabilities, hard resource limits, and an instant cross-thread kill switch.",
  keywords: [
    "AI agent security",
    "WebAssembly sandbox",
    "Wasmtime runtime",
    "Software fault isolation",
    "Python agent execution",
    "Safe code sandbox",
  ],
  manifest: "/manifest.json",
  icons: {
    icon: [
      { url: "/favicon.svg", type: "image/svg+xml" },
      { url: "/favicon.png", sizes: "64x64", type: "image/png" },
    ],
    apple: [
      { url: "/apple-icon.png", sizes: "180x180", type: "image/png" },
    ],
  },
  openGraph: {
    title: "Boxwren — Sandboxed WebAssembly Execution Runtime for AI Agents",
    description: "Contain untrusted AI agent code at 1 µs overhead. In-process Wasmtime isolation, deny-by-default capabilities, and instant cross-thread kill switch.",
    url: "https://boxwren.dev",
    siteName: "Boxwren",
    locale: "en_US",
    type: "website",
    images: [
      {
        url: "/icon-512.png",
        width: 512,
        height: 512,
        alt: "Boxwren Runtime Enclave Emblem",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "Boxwren — Sandboxed WebAssembly Execution Runtime for AI Agents",
    description: "Contain untrusted AI agent code at 1 µs overhead. In-process Wasmtime isolation, deny-by-default capabilities, and instant cross-thread kill switch.",
    images: ["/icon-512.png"],
  },
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <head>
        <meta name="color-scheme" content="dark" />
      </head>
      <body>
        <Navbar />

        <main className="site-wrap">{children}</main>

        <footer className="site-footer">
          <div className="footer-inner">
            <div className="footer-brand-col">
              <div style={{ display: "flex", alignItems: "center", gap: "0.6rem" }}>
                <BoxwrenLogo size={22} withGlow />
                <p className="font-semibold text-primary" style={{ letterSpacing: "-0.01em" }}>
                  Boxwren Runtime
                </p>
              </div>
              <p className="muted" style={{ fontSize: "0.82rem", marginTop: "0.5rem", maxWidth: "42ch", lineHeight: 1.5 }}>
                Software Fault Isolation for autonomous agent workflows. In-process Wasmtime runtime dual-licensed MIT / Apache-2.0.
              </p>
            </div>

            <div className="footer-links" aria-label="Footer Navigation">
              <a href="https://github.com/rejisterjack/boxwren" target="_blank" rel="noreferrer">
                GitHub Repository
              </a>
              <a href="https://crates.io/crates/boxwren" target="_blank" rel="noreferrer">
                crates.io (0.2.4)
              </a>
              <a href="https://pypi.org/project/boxwren-py/" target="_blank" rel="noreferrer">
                PyPI (boxwren-py)
              </a>
              <a href="https://docs.rs/boxwren" target="_blank" rel="noreferrer">
                docs.rs API Reference
              </a>
              <Link href="/dashboard">Operator Dashboard</Link>
            </div>
          </div>
        </footer>
      </body>
    </html>
  );
}
