"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";

interface NavItem {
  href: string;
  label: string;
}

const NAV_ITEMS: NavItem[] = [
  { href: "/#sandbox", label: "Interactive Enclave" },
  { href: "/#benchmarks", label: "Benchmarks" },
  { href: "/#capabilities", label: "Capabilities" },
  { href: "/#architecture", label: "Safety Stack" },
  { href: "/#threat-model", label: "Threat Model" },
  { href: "/#install", label: "Install" },
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
      const scrollY = window.scrollY + 120;
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
    <header className="site-header">
      <div className="nav-inner">
        {/* Brand Group */}
        <div className="brand-group">
          <Link href="/" className="brand-logo" aria-label="Boxwren Homepage" onClick={handleNavClick}>
            <span className="brand-mark" aria-hidden="true" />
            <span>Boxwren</span>
          </Link>
          <span className="version-tag">
            <span className="version-dot" aria-hidden="true" />
            v0.2.3 Enforced
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

        {/* Action Controls & Mobile Hamburger */}
        <div className="nav-actions">
          <a
            href="https://github.com/rejisterjack/boxwren"
            target="_blank"
            rel="noreferrer"
            className="btn btn-ghost btn-sm nav-github-btn"
            aria-label="View Boxwren repository on GitHub"
          >
            GitHub ★
          </a>
          <Link
            href="/dashboard"
            className={`btn btn-sm ${pathname === "/dashboard" ? "btn-secondary" : "btn-primary"}`}
            aria-current={pathname === "/dashboard" ? "page" : undefined}
          >
            {pathname === "/dashboard" ? "Dashboard" : "Dashboard"}
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

      {/* Mobile Drawer Backdrop */}
      {mobileMenuOpen && (
        <div
          className="mobile-backdrop"
          onClick={() => setMobileMenuOpen(false)}
          aria-hidden="true"
        />
      )}

      {/* Mobile Navigation Drawer */}
      <div
        id="mobile-nav-drawer"
        className={`mobile-drawer ${mobileMenuOpen ? "drawer-open" : ""}`}
        aria-hidden={!mobileMenuOpen}
      >
        <div className="mobile-drawer-header">
          <span className="mobile-drawer-title">Navigation</span>
          <span className="version-tag">v0.2.3</span>
        </div>
        <nav className="mobile-nav-list" aria-label="Mobile Navigation">
          {NAV_ITEMS.map((item) => (
            <Link
              key={item.href}
              href={item.href}
              onClick={handleNavClick}
              className={`mobile-nav-item ${activeSection === item.href ? "active" : ""}`}
            >
              <span>{item.label}</span>
              <span className="mobile-nav-arrow" aria-hidden="true">→</span>
            </Link>
          ))}
        </nav>
        <div className="mobile-drawer-actions">
          <a
            href="https://github.com/rejisterjack/boxwren"
            target="_blank"
            rel="noreferrer"
            className="btn btn-secondary"
            style={{ width: "100%" }}
            onClick={handleNavClick}
          >
            GitHub Repository ★
          </a>
          <Link
            href="/dashboard"
            className="btn btn-primary"
            style={{ width: "100%" }}
            onClick={handleNavClick}
          >
            Open Operator Dashboard
          </Link>
        </div>
      </div>
    </header>
  );
}
