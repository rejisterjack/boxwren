import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Terms — Boxwren",
  description: "What Boxwren does and does not isolate.",
};

export default function TermsPage() {
  return (
    <article style={{ maxWidth: "68ch", padding: "2.5rem 0 4rem" }}>
      <h1>Terms</h1>
      <p style={{ margin: "1rem 0" }}>
        Boxwren runs untrusted WebAssembly inside Wasmtime. Filesystem, network, and environment
        access stay closed unless a grant says otherwise. Fuel, memory, and wall-clock limits stop
        a guest that runs too long. This is software fault isolation, not a virtual machine.
      </p>
      <p style={{ margin: "1rem 0" }}>
        You choose the grants. A grant of <code>/</code> or your home directory is broad, and
        Boxwren warns instead of rewriting it. The hosted site ignores grant fields and clamps
        fuel, memory, and wall time. Live kill on the hosted site is refused; use the local
        server for that.
      </p>
      <p className="muted" style={{ margin: "1rem 0" }}>
        Wasmtime bugs, Spectre-class leaks, and a guest that fails before the filesystem hook are
        outside the boundary this sandbox claims.
      </p>
    </article>
  );
}
