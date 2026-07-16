/** Just enough highlighting to read a config at a glance — keys, strings,
 *  numbers, comments. A tokeniser, not a parser: it never has to *understand*
 *  the file, only make a wall of grey stop being a wall of grey, and shipping a
 *  syntax library to colour twenty lines is not a trade worth making. */
const RULES: [RegExp, string][] = [
  [/(^|\s)(#.*)$/gm, "text-faint"], // yaml comment
  [/"([^"\\]|\\.)*"(?=\s*:)/g, "text-sky-ink"], // json key
  [/^\s*[\w.-]+(?=:)/gm, "text-sky-ink"], // yaml key
  [/"([^"\\]|\\.)*"/g, "text-gold-ink"], // string
  [/\b\d+\b/g, "text-grape-ink"], // number
];

type Tok = { text: string; cls?: string };

export function Code({ body }: { body: string }) {
  return (
    <pre className="max-h-64 overflow-auto p-2 text-[10px] leading-relaxed text-ink">
      {paint(body).map((t, i) => (
        <span key={i} className={t.cls}>
          {t.text}
        </span>
      ))}
    </pre>
  );
}

/** Walk the rules in order, keeping whatever a rule has already claimed. */
function paint(src: string): Tok[] {
  let toks: Tok[] = [{ text: src }];
  for (const [re, cls] of RULES) {
    toks = toks.flatMap((tok) => (tok.cls ? [tok] : split(tok.text, re, cls)));
  }
  return toks;
}

function split(text: string, re: RegExp, cls: string): Tok[] {
  const out: Tok[] = [];
  let last = 0;
  for (const m of text.matchAll(re)) {
    const at = m.index!;
    if (at > last) out.push({ text: text.slice(last, at) });
    out.push({ text: m[0], cls });
    last = at + m[0].length;
  }
  if (last < text.length) out.push({ text: text.slice(last) });
  return out;
}
