/** Holds issued tokens. */
export class TokenStore {
  private tokens: string[] = [];

  lookup(token: string): string | undefined {
    return this.tokens.find((t) => t === token);
  }

  insert(token: string): void {
    this.tokens.push(token);
  }
}

export function now(): number {
  return 42;
}
