import "./style.css";

const offline = document.querySelector<HTMLElement>("#offline");
const setNetworkState = () => {
  if (offline) offline.hidden = navigator.onLine;
};
window.addEventListener("online", setNetworkState);
window.addEventListener("offline", setNetworkState);
setNetworkState();

if ("serviceWorker" in navigator && import.meta.env.PROD) {
  window.addEventListener("load", () => navigator.serviceWorker.register("/sw.js").catch(() => undefined));
}

const plainText = (element: Element) => element.textContent?.replace(/\n\s+/g, "\n") ?? "";
document.querySelectorAll<HTMLButtonElement>("[data-copy]").forEach((button) => {
  button.addEventListener("click", async () => {
    const source = document.getElementById(button.dataset.copy ?? "");
    const status = document.querySelector<HTMLElement>("#copy-status");
    if (!source) return;
    try {
      await navigator.clipboard.writeText(plainText(source));
      button.textContent = "Copied";
      if (status) status.textContent = "Configuration copied to clipboard.";
    } catch {
      button.textContent = "Select code";
      window.getSelection()?.selectAllChildren(source);
      if (status) status.textContent = "Clipboard was unavailable. The configuration is selected for copying.";
    }
    window.setTimeout(() => { button.textContent = "Copy config"; }, 1800);
  });
});

type DemoKind = "pass" | "fail";
const terminal = document.querySelector<HTMLElement>("#terminal");
const state = document.querySelector<HTMLElement>("#demo-state");
const hint = document.querySelector<HTMLElement>("#demo-hint");
const title = document.querySelector<HTMLElement>("#receipt-title");
const hash = document.querySelector<HTMLElement>("#receipt-hash");
const cleanup = document.querySelector<HTMLElement>("#receipt-cleanup");
const exit = document.querySelector<HTMLElement>("#receipt-exit");
const stamp = document.querySelector<HTMLElement>("#stamp");
const reduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
let runId = 0;

const wait = (ms: number) => new Promise((resolve) => window.setTimeout(resolve, reduced ? 0 : ms));
const setTerminal = (lines: string[]) => {
  if (terminal) terminal.innerHTML = `<span class="prompt">$</span> restore-drill run --config fixture.toml\n${lines.join("\n")}`;
};

async function runDemo(kind: DemoKind) {
  const currentRun = ++runId;
  document.querySelectorAll<HTMLButtonElement>("#demo button").forEach((button) => { button.disabled = true; });
  if (state) state.textContent = "Drill running";
  if (hint) hint.textContent = "A fresh temporary target has been created.";
  if (title) title.textContent = "Receipt pending";
  if (hash) hash.textContent = "calculating…";
  if (cleanup) cleanup.textContent = "pending";
  if (exit) exit.textContent = "—";
  if (stamp) { stamp.textContent = "Checking"; stamp.className = "stamp empty"; }
  const lines = ["<span class=\"running\">CREATE</span>  temporary target /tmp/restore-drill-••••••"];
  setTerminal(lines);
  await wait(460);
  if (currentRun !== runId) return;
  lines.push("<span class=\"running\">RESTORE</span> Documents/tax.pdf");
  setTerminal(lines);
  await wait(520);
  if (currentRun !== runId) return;

  if (kind === "fail") {
    lines.push("<span class=\"fail-text\">MISSING</span> Documents/tax.pdf");
    lines.push("<span class=\"pass-text\">CLEAN</span>   temporary target removed");
    lines.push("<span class=\"fail-text\">FAIL</span>    sample was not restored  [exit 1]");
    setTerminal(lines);
    if (state) state.textContent = "Failure caught loudly";
    if (hint) hint.textContent = "Next: confirm the backup includes this path, then rerun.";
    if (title) title.textContent = "Recovery not proven";
    if (hash) hash.textContent = "not available";
    if (cleanup) cleanup.textContent = "complete";
    if (exit) exit.textContent = "1 / failed";
    if (stamp) { stamp.textContent = "Failed"; stamp.className = "stamp failed"; }
  } else {
    lines.push("<span class=\"pass-text\">HASH</span>    sha256 8b51…a02f matched");
    await wait(360);
    lines.push("<span class=\"pass-text\">OPEN</span>    pdftotext accepted file");
    lines.push("<span class=\"pass-text\">CLEAN</span>   temporary target removed");
    lines.push("<span class=\"pass-text\">PASS</span>    receipt chain advanced  [exit 0]");
    setTerminal(lines);
    if (state) state.textContent = "Recovery proven";
    if (hint) hint.textContent = "Next drill is due in 30 days. No alert needed.";
    if (title) title.textContent = "All checks passed";
    if (hash) hash.textContent = "8b51…a02f / match";
    if (cleanup) cleanup.textContent = "complete";
    if (exit) exit.textContent = "0 / passed";
    if (stamp) { stamp.textContent = "Verified"; stamp.className = "stamp passed"; }
  }
  document.querySelectorAll<HTMLButtonElement>("#demo button").forEach((button) => { button.disabled = false; });
}

document.querySelector("#run-pass")?.addEventListener("click", () => runDemo("pass"));
document.querySelector("#run-fail")?.addEventListener("click", () => runDemo("fail"));
