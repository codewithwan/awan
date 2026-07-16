import { useState } from "react";
import { DEFAULT_STORY, type Scene } from "./lib/acts";
import { BLANK, type Identity } from "./lib/config";
import { useDraft } from "./lib/store";
import { Stepper, STEPS } from "./ui/Stepper";
import { Button } from "./ui/Button";
import { PixelIcon } from "./ui/PixelIcon";
import { SkinToggle } from "./ui/SkinToggle";
import { GithubMark } from "./ui/GithubMark";
import { StepIdentity } from "./steps/StepIdentity";
import { StepStory } from "./steps/StepStory";
import { StepExport } from "./steps/StepExport";
import { Studio, slug } from "./character/Studio";
import { starter, } from "./character/starters";
import { toToml, type Character } from "./lib/spec";

/** The shell: which step you're on, and the two pieces of state the steps
 *  share. Everything that draws lives somewhere else. */
export function App() {
  const [at, setAt] = useState(0);
  // the draft survives a refresh — rewriting seven captions because you
  // reached for reload out of habit is a page you don't come back to
  const [story, setStory] = useDraft<Scene[]>("story", DEFAULT_STORY);
  const [id, setId] = useDraft<Identity>("identity", BLANK);
  const [cast, setCast] = useDraft<string>("cast", "awan");
  const [beat, setBeat] = useState(-1);
  const [solo, setSolo] = useState(-1);
  // a character you drew, kept with the draft — nobody should lose a drawing to
  // a reload either
  const [mine, setMine] = useDraft<Character | null>("mine", null);
  const [drawing, setDrawing] = useState(false);

  const drawn = mine
    ? { id: "mine", label: mine.name || "Yours", blurb: mine.description || "drawn by you",
        toml: toToml(mine), path: `characters/${slug(mine.name)}.toml` }
    : undefined;

  return (
    <div className="mx-auto flex min-h-screen max-w-6xl flex-col gap-6 px-4 py-8">
      <Header />
      <Stepper at={at} onGo={setAt} />

      <main className="min-w-0 flex-1">
        {drawing && mine ? (
          <Studio char={mine} onChange={setMine} onClose={() => setDrawing(false)} />
        ) : (
          <>
        {at === 0 && <StepIdentity id={id} onChange={setId} />}
        {at === 1 && (
          <StepStory
            story={story}
            beat={beat}
            cast={cast}
            solo={solo}
            id={id}
            drawn={drawn}
            onStory={setStory}
            onBeat={setBeat}
            onCast={setCast}
            onSolo={setSolo}
            onDraw={() => {
              if (!mine) setMine(starter("blob"));
              setCast("mine");
              setDrawing(true);
            }}
          />
        )}
        {at === 2 && <StepExport id={id} story={story} cast={cast} drawn={drawn} />}
          </>
        )}
      </main>

      {/* Neither arrow appears where it has nowhere to go. A button whose only
          job is to be greyed out is furniture, and the last step's Next was
          worse than furniture: it implied a fourth step that doesn't exist. */}
      <nav className={`flex gap-2 ${drawing ? "hidden" : ""}`}>
        {at > 0 && <Button onClick={() => setAt((i) => i - 1)}>‹ back</Button>}
        {at < STEPS.length - 1 && (
          <Button tone="lime" onClick={() => setAt((i) => i + 1)} className="ml-auto">
            next ›
          </Button>
        )}
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
      <div className="ml-auto flex gap-2">
        <SkinToggle />
        <a
          href="https://github.com/codewithwan/awan"
          className="nb-btn flex items-center gap-2 bg-slab px-3 py-1.5 text-[10px] uppercase text-ink"
        >
          <GithubMark />
          source
        </a>
      </div>
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
