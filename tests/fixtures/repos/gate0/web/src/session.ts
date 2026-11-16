import { TokenStore, now } from "./store";

export class Session {
  issued: number;

  constructor(public token: string) {
    this.issued = now();
  }

  isFresh(): boolean {
    return this.issued + 10 > now();
  }
}

/** Issues a new session when the token is known. */
export function refresh(store: TokenStore, token: string): Session | undefined {
  const found = store.lookup(token);
  if (found === undefined) return undefined;
  const s = new Session(found);
  return s.isFresh() ? s : undefined;
}
