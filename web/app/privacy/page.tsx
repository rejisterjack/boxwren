import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Privacy — Boxwren",
  description: "What the Boxwren site stores when you create an account.",
};

export default function PrivacyPage() {
  return (
    <article style={{ maxWidth: "68ch", padding: "2.5rem 0 4rem" }}>
      <h1>Privacy</h1>
      <p className="muted" style={{ margin: "1rem 0" }}>
        The local CLI and the Rust, Python, and Go libraries run on your machine. They do not
        create an account and they do not send a module anywhere.
      </p>
      <p style={{ margin: "1rem 0" }}>
        The hosted site stores the email address you submit, a hash of each API key, and the
        execution reports for runs made with that key. The raw API key is returned once. The
        dashboard keeps it in this browser under the <code>boxwren_api_key</code> localStorage
        entry until you sign out.
      </p>
      <p style={{ margin: "1rem 0" }}>
        There is no password, session cookie, or mailing list. Sign out removes the key from this
        browser. It does not delete the account row.
      </p>
    </article>
  );
}
