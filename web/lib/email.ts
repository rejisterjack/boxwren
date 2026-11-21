export function validEmail(email: string): boolean {
  return (
    email.length <= 320 &&
    email.includes("@") &&
    !email.startsWith("@") &&
    !email.endsWith("@") &&
    !/\s/.test(email)
  );
}