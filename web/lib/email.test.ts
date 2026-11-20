import test from "node:test";
import assert from "node:assert/strict";

import { validEmail } from "./email.ts";

test("validEmail matches server rules", () => {
  assert.equal(validEmail("user@example.com"), true);
  assert.equal(validEmail("@bad.com"), false);
  assert.equal(validEmail("bad@"), false);
  assert.equal(validEmail("spaces in@x.com"), false);
  assert.equal(validEmail("üser@example.com"), true);
});