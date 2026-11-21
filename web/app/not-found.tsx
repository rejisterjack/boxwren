import type { Metadata } from "next";
import Link from "next/link";

export const metadata: Metadata = {
  title: "Not found — Boxwren",
};

export default function NotFound() {
  return (
    <section className="content-section" style={{ textAlign: "center", padding: "4rem 1rem" }}>
      <span className="badge-kicker">404</span>
      <h1 className="section-title" style={{ marginTop: "0.75rem" }}>
        Page not found
      </h1>
      <p className="section-lead" style={{ maxWidth: "48ch", margin: "0.75rem auto 1.5rem" }}>
        This path does not exist in the Boxwren site. Return to the runtime overview or open the operator dashboard.
      </p>
      <div className="cta-row" style={{ justifyContent: "center" }}>
        <Link href="/" className="btn btn-primary">
          Home
        </Link>
        <Link href="/dashboard" className="btn btn-secondary">
          Dashboard
        </Link>
      </div>
    </section>
  );
}