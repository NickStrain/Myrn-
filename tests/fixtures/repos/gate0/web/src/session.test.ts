import { refresh } from "./session";
import { TokenStore } from "./store";

describe("refresh", () => {
  it("returns a session for a known token", () => {
    const store = new TokenStore();
    store.insert("abc");
    expect(refresh(store, "abc")).toBeDefined();
  });
});
