import { useState } from "react";
import { DEFAULT_STORY, type Scene } from "./lib/acts";
import { BLANK, type Identity } from "./lib/config";
import { Stepper, STEPS } from "./ui/Stepper";
import { Button } from "./ui/Button";
import { PixelIcon } from "./ui/PixelIcon";
import { StepIdentity } from "./steps/StepIdentity";
import { StepStory } from "./steps/StepStory";
import { StepExport } from "./steps/StepExport";

/** The shell: which step you're on, and the two pieces of state the steps
 *  share. Everything that draws lives somewhere else. */
export function App() {
  const [at, setAt] = useState(0);
  const [story, setStory] = useState<Scene[]>(DEFAULT_STORY);
  const [id, setId] = useState<Identity>(BLANK);
  const [beat, setBeat] = useState(-1);

  return (
    <div className="mx-auto flex min-h-screen max-w-6xl flex-col gap-6 px-4 py-8">
      <Header />
      <Stepper at={at} onGo={setAt} />

      <main className="flex-1">
        {at === 0 && <StepIdentity id={id} onChange={setId} />}
        {at === 1 && <StepStory story={story} beat={beat} onStory={setStory} onBeat={setBeat} />}
        {at === 2 && <StepExport id={id} story={story} />}
      </main>

      <nav className="flex gap-2">
        <Button onClick={() => setAt((i) => i - 1)} disabled={at === 0}>
          ‹ back
        </Button>
        <Button tone="lime" onClick={() => setAt((i) => i + 1)} disabled={at === STEPS.length - 1} className="ml-auto">
          next ›
        </Button>
      </nav>

      <Footer />
    </div>
  );
}

function Header() {
  return (
    <header className="flex flex-wrap items-center gap-3">
      <PixelIcon id="cloud" size={36} />
      <div>
        <h1 className="text-2xl uppercase tracking-tight text-ink">awan</h1>
        <p className="text-[10px] text-mute">a tiny living character for your GitHub profile</p>
      </div>
      <a href="https://github.com/codewithwan/awan" className="nb-btn ml-auto bg-slab px-3 py-1.5 text-[10px] uppercase text-ink">
        source ↗
      </a>
    </header>
  );
}

function Footer() {
  return (
    <footer className="border-t-3 border-line pt-4 text-[10px] leading-relaxed text-mute">
      <p>
        The preview is the engine itself, compiled to wasm — same ticks, same cells, same font, same
        1056×416 canvas as the file CI commits. Its numbers are stand-ins: the calendar behind the
        year wall lives in an API that wants a token, and no cartoon is worth pasting a token into a
        web page for. CI has one of its own.
      </p>
      <p className="mt-2">
        Nothing is stored and nothing is sent — there's no server here to send it to. That's also why
        this page can't break your README: your config lives in your repo, and the workflow runs
        there.
      </p>
    </footer>
  );
}
