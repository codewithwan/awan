import { useEffect, useState } from "react";

/** State that survives a refresh.
 *
 *  Somebody rewrites seven captions, reaches for the reload out of habit, and
 *  loses the lot — that is a page you don't come back to. It stays in the
 *  browser: no account to make, nothing sent anywhere, and clearing your
 *  history clears it, which is exactly what someone clearing their history
 *  means.
 */
const KEY = "awan.draft.v1";

type Draft = Record<string, unknown>;

const read = (): Draft => {
  try {
    return JSON.parse(localStorage.getItem(KEY) ?? "{}");
  } catch {
    return {}; // a corrupt draft is not worth a blank page
  }
};

export function useDraft<T>(field: string, initial: T) {
  const [value, setValue] = useState<T>(() => (read()[field] as T) ?? initial);

  useEffect(() => {
    try {
      localStorage.setItem(KEY, JSON.stringify({ ...read(), [field]: value }));
    } catch {
      // private mode, or a full quota — losing the draft beats losing the page
    }
  }, [field, value]);

  return [value, setValue] as const;
}

export const clearDraft = () => localStorage.removeItem(KEY);
