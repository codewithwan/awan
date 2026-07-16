import { useState } from "react";
import { DEFAULT_STORY, type Scene } from "./acts";
import { Preview } from "./Preview";
import { SceneList } from "./SceneList";
import { Shelf } from "./Shelf";
import { Meter } from "./Meter";
import { Identity } from "./Identity";
import { Output, type Identity as Id } from "./Output";

const BLANK: Id = {
  handle: "",
  name: "",
  role: "",
  location: "",
  stack: "",
  song: "",
  artist: "",
  lyrics: [],
};

export function App() {
  const [story, setStory] = useState<Scene[]>(DEFAULT_STORY);
  const [id, setId] = useState<Id>(BLANK);
  const [beat, setBeat] = useState(-1);

  return (
    <main className="mx-auto max-w-6xl px-4 py-8">
      <Header />

      <div className="mt-6 grid gap-6 lg:grid-cols-[1.3fr_1fr]">
        <div className="flex flex-col gap-4">
          <Preview story={story} onBeat={setBeat} />
          <Meter story={story} />
        </div>

        <div className="flex flex-col gap-4">
          <Panel title="The story" hint="drag to reorder">
            <SceneList story={story} playing={beat} onChange={setStory} />
          </Panel>
          <Panel title="Add a beat">
            <Shelf onAdd={(s) => setStory([...story, s])} />
          </Panel>
          <Panel title="About you">
            <Identity id={id} onChange={setId} />
          </Panel>
        </div>
      </div>

      <div className="mt-6">
        <Panel title="Take it home" hint="three files, no secrets">
          <Output id={id} story={story} />
        </Panel>
      </div>

      <Footer />
    </main>
  );
}

function Header() {
  return (
    <header className="flex flex-wrap items-end gap-x-4 gap-y-2">
      <h1 className="text-3xl text-ink">
        awan <span className="text-cloud">☁</span>
      </h1>
      <p className="text-sm text-mute">
        a tiny living character for your GitHub profile
      </p>
      <a
        href="https://github.com/codewithwan/awan"
        className="px-btn ml-auto bg-panel px-3 py-1 text-xs text-ink"
      >
        source ↗
      </a>
    </header>
  );
}

function Panel({
  title,
  hint,
  children,
}: {
  title: string;
  hint?: string;
  children: React.ReactNode;
}) {
  return (
    <section className="px-box p-3">
      <div className="mb-3 flex items-baseline gap-2 border-b-2 border-edge pb-2">
        <h2 className="text-sm text-gold">{title}</h2>
        {hint && <span className="text-[10px] text-edge">{hint}</span>}
      </div>
      {children}
    </section>
  );
}

function Footer() {
  return (
    <footer className="mt-10 border-t-2 border-edge pt-4 text-xs text-mute">
      <p>
        The preview runs the real engine, compiled to wasm — the frames above are the frames CI
        draws. Its numbers are stand-ins: the calendar behind the year wall lives in an API that
        wants a token, and no cartoon is worth pasting a token into a web page for. CI has one of
        its own, and fills in yours.
      </p>
      <p className="mt-2">
        Nothing here is stored, and nothing is sent anywhere — the page has no server to send it to.
        That's also why it can't break your README: your config lives in your repo, and the workflow
        runs there.
      </p>
    </footer>
  );
}
