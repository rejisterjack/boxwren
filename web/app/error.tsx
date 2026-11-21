"use client";

import Link from "next/link";
import { useEffect } from "react";

export default function Error({
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    document.title = "Error — Boxwren";
  }, []);
  return (
    <section className="content-section" style={{ textAlign: "center", padding: "4rem 1rem" }}>
      <span className="badge-kicker">500</span>
      <h1 className="section-title" style={{ marginTop: "0.75rem" }}>
        Something went wrong
      </h1>
      <p className="section-lead" style={{ maxWidth: "48ch", margin: "0.75rem auto 1.5rem" }}>
        The page failed while rendering. The details stay on the server. You can try again or return home.
      </p>
      <div className="cta-row" style={{ justifyContent: "center" }}>
        <button className="btn btn-primary" type="button" onClick={() => reset()}>
          Try again
        </button>
        <Link href="/" className="btn btn-secondary">
          Home
        </Link>
      </div>
    </section>
  );
}
