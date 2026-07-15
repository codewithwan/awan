export interface Options {
  /** Path to a character TOML spec. Defaults to the built-in buddy. */
  character?: string;
}

export interface Handle {
  /** Stop the running character. */
  stop(): void;
}

export interface Companion extends Handle {
  /** Feed an event (e.g. "cmd.ok", "task.done") to the ambient companion. */
  emit(event: string): void;
}

/** Absolute path to the resolved `awan` binary. */
export const binPath: string;

/** Play a character's one-shot reaction to an event, then return (blocking). */
export function react(event: string, opts?: Options): void;

/** Run the "working…" loop with a caption; call `.stop()` when done. */
export function busy(label: string, opts?: Options): Handle;

/** Start an ambient companion; `.emit(event)` feeds it, `.stop()` ends it. */
export function watch(opts?: Options): Companion;
