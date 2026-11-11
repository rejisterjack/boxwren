"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import BoxwrenLogo from "./BoxwrenLogo";

interface NavItem {
  href: string;
  label: string;
}

const NAV_ITEMS: NavItem[] = [
  { href: "/#sandbox", label: "Live Enclave" },
  { href: "/#benchmarks", label: "Benchmarks" },
  { href: "/#capabilities", label: "Capabilities" },
  { href: "/#architecture", label: "Safety Stack" },
  { href: "/#threat-model", label: "Threat Model" },
  { href: "/#install", label: "Quickstart" },
];

export default function Navbar() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [activeSection, setActiveSection] = useState<string>("");
  const pathname = usePathname();

  useEffect(() => {
    if (pathname !== "/") {
      setActiveSection("");
      return;
    }

    const sectionIds = ["sandbox", "benchmarks", "capabilities", "architecture", "threat-model", "install"];
    const handleScroll = () => {
      const scrollY = window.scrollY + 140;
      for (let i = sectionIds.length - 1; i >= 0; i--) {
        const el = document.getElementById(sectionIds[i]);
        if (el && el.offsetTop <= scrollY) {
          setActiveSection(`/#${sectionIds[i]}`);
          return;
        }
      }
      setActiveSection("");
    };

    window.addEventListener("scroll", handleScroll, { passive: true });
    handleScroll();
    return () => window.removeEventListener("scroll", handleScroll);
  }, [pathname]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") setMobileMenuOpen(false);
    };
    if (mobileMenuOpen) {
      document.body.style.overflow = "hidden";
      window.addEventListener("keydown", handleKeyDown);
    } else {
      document.body.style.overflow = "";
    }
    return () => {
      document.body.style.overflow = "";
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [mobileMenuOpen]);

  const handleNavClick = () => {
    setMobileMenuOpen(false);
  };

  return (
    <header className="site-header" role="banner">
      <div className="nav-inner">
        {/* Brand Group with Bespoke Vector Emblem */}
        <div className="brand-group">
          <Link href="/" className="brand-logo" aria-label="Boxwren Homepage" onClick={handleNavClick}>
            <BoxwrenLogo size={32} withGlow className="brand-logo-icon" />
            <span className="brand-name">Boxwren</span>
          </Link>
          <span className="version-tag" title="Enforced Runtime Spec v0.2.3">
            <span className="version-dot" aria-hidden="true" />
            v0.2.3 Wasmtime
          </span>
        </div>

        {/* Desktop Navigation Links */}
        <nav className="nav-links desktop-nav" aria-label="Main Navigation">
          {NAV_ITEMS.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              className={`nav-link ${activeSection === item.href ? "active" : ""}`}
            >
              {item.label}
            </Link>
          ))}
        </nav>

        {/* Action Controls & External GitHub */}
        <div className="nav-actions">
          <a
            href="https://github.com/rejisterjack/boxwren"
            target="_blank"
            rel="noreferrer"
            className="btn btn-ghost btn-sm nav-github-btn"
            aria-label="Star Boxwren on GitHub"
          >
            <svg
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="currentColor"
              aria-hidden="true"
              style={{ marginRight: 6 }}
            >
              <path fillRule="evenodd" clipRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
            </svg>
            GitHub ★
          </a>
          <Link
            href="/dashboard"
            className={`btn btn-sm ${pathname === "/dashboard" ? "btn-secondary" : "btn-primary"}`}
            aria-current={pathname === "/dashboard" ? "page" : undefined}
          >
            Dashboard
          </Link>

          {/* Mobile Menu Hamburger Button */}
          <button
            type="button"
            className={`mobile-menu-btn ${mobileMenuOpen ? "open" : ""}`}
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            aria-expanded={mobileMenuOpen}
            aria-controls="mobile-nav-drawer"
            aria-label={mobileMenuOpen ? "Close navigation menu" : "Open navigation menu"}
          >
            <span className="hamburger-line" aria-hidden="true" />
            <span className="hamburger-line" aria-hidden="true" />
            <span className="hamburger-line" aria-hidden="true" />
          </button>
        </div>
      </div>

      {/* Mobile Menu Drawer Overlay */}
      {mobileMenuOpen && (
        <div id="mobile-nav-drawer" className="mobile-nav-drawer" role="dialog" aria-modal="true">
          <div className="mobile-nav-content">
            <nav className="mobile-nav-links" aria-label="Mobile Navigation">
              {NAV_ITEMS.map((item) => (
                <Link
                  key={item.href}
                  href={item.href}
                  className={`mobile-nav-link ${activeSection === item.href ? "active" : ""}`}
                  onClick={handleNavClick}
                >
                  {item.label}
                </Link>
              ))}
              <div className="mobile-nav-divider" />
              <Link
                href="/dashboard"
                className="btn btn-primary"
                onClick={handleNavClick}
                style={{ width: "100%", justifyContent: "center", marginTop: "1rem" }}
              >
                Launch Playground Dashboard
              </Link>
            </nav>
          </div>
        </div>
      )}
    </header>
  );
}
