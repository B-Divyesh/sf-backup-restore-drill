import "./style.css";

const offline = document.querySelector<HTMLElement>("#offline");
const setNetworkState = () => {
  if (offline) offline.hidden = navigator.onLine;
};
setNetworkState();
window.addEventListener("online", setNetworkState);
window.addEventListener("offline", setNetworkState);

if (location.pathname === "/" && new URLSearchParams(location.search).get("demo") === "1") {
  location.replace("/demo/?demo=1");
}

if ("serviceWorker" in navigator) {
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
      button.textContent = "Copied configuration";
      if (status) status.textContent = "Configuration copied to clipboard.";
    } catch {
      button.textContent = "Select configuration";
      window.getSelection()?.selectAllChildren(source);
      if (status) status.textContent = "Clipboard was unavailable. The configuration is selected for copying.";
    }
    window.setTimeout(() => { button.textContent = "Copy configuration"; }, 1800);
  });
});

const routeStatus = document.querySelector<HTMLElement>("#route-status");

function headingFor(target: HTMLElement) {
  return target.matches("h1, h2, h3") ? target : target.querySelector<HTMLElement>("h1, h2, h3");
}

function focusDestination(target: HTMLElement, scroll: boolean) {
  const heading = headingFor(target) ?? target;
  if (scroll) target.scrollIntoView();
  heading.tabIndex = -1;
  heading.focus({ preventScroll: true });
  if (routeStatus) routeStatus.textContent = heading.textContent?.trim() ?? "Section changed";
}

function focusHashDestination(scroll = false) {
  if (location.hash) {
    try {
      const target = document.querySelector<HTMLElement>(location.hash);
      if (target) {
        focusDestination(target, scroll);
        return;
      }
    } catch {
      // A malformed fragment should leave the document usable at its heading.
    }
  }
  const heading = document.querySelector<HTMLElement>("main h1");
  if (heading) focusDestination(heading, false);
}

document.querySelectorAll<HTMLAnchorElement>('a[href^="#"]').forEach((link) => {
  link.addEventListener("click", (event) => {
    const target = document.querySelector<HTMLElement>(link.getAttribute("href") ?? "");
    if (!target) return;
    event.preventDefault();
    history.pushState(null, "", link.getAttribute("href"));
    focusDestination(target, true);
  });
});

window.addEventListener("popstate", () => focusHashDestination(true));
window.addEventListener("hashchange", () => focusHashDestination(true));
window.addEventListener("pageshow", (event) => {
  if (event.persisted) focusHashDestination(false);
});
window.requestAnimationFrame(() => focusHashDestination(false));

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
  if (terminal) terminal.innerHTML = `<span class="prompt">$</span> restore-drill demo\n${lines.join("\n")}`;
};

async function runDemo(kind: DemoKind) {
  if (!terminal) return;
  const currentRun = ++runId;
  document.querySelectorAll<HTMLButtonElement>(".demo-controls button").forEach((button) => { button.disabled = true; });
  if (state) state.textContent = "Sample restore running";
  if (hint) hint.textContent = "The command created a new temporary restore folder.";
  if (title) title.textContent = "Receipt pending";
  if (hash) hash.textContent = "checking…";
  if (cleanup) cleanup.textContent = "pending";
  if (exit) exit.textContent = "—";
  if (stamp) { stamp.textContent = "Checking"; stamp.className = "stamp empty"; }
  const lines = ["<span class=\"running\">RESTORE</span> Documents/quarterly-tax-notes.txt"];
  setTerminal(lines);
  await wait(220);
  if (currentRun !== runId) return;
  if (kind === "fail") {
    lines.push("<span class=\"fail-text\">MISSING</span> Documents/quarterly-tax-notes.txt", "<span class=\"pass-text\">CLEAN</span>   temporary restore folder removed", "<span class=\"fail-text\">FAIL</span>    sample was not restored  [exit 1]");
    setTerminal(lines);
    if (state) state.textContent = "Missing file detected";
    if (hint) hint.textContent = "Confirm the backup includes this path, then run the drill again.";
    if (title) title.textContent = "Recovery not verified";
    if (hash) hash.textContent = "not available";
    if (cleanup) cleanup.textContent = "complete";
    if (exit) exit.textContent = "1 / failed";
    if (stamp) { stamp.textContent = "Failed"; stamp.className = "stamp failed"; }
  } else {
    lines.push("<span class=\"pass-text\">HASH</span>    SHA-256 fingerprint matched", "<span class=\"pass-text\">OPEN</span>    file check passed", "<span class=\"pass-text\">CLEAN</span>   temporary restore folder removed", "<span class=\"pass-text\">PASS</span>    hash-linked receipt written  [exit 0]");
    setTerminal(lines);
    if (state) state.textContent = "Sample restored and verified";
    if (hint) hint.textContent = "The temporary restore folder was removed before the receipt was written.";
    if (title) title.textContent = "All checks passed";
    if (hash) hash.textContent = "296d…fe48 / match";
    if (cleanup) cleanup.textContent = "complete";
    if (exit) exit.textContent = "0 / passed";
    if (stamp) { stamp.textContent = "Verified"; stamp.className = "stamp passed"; }
  }
  document.querySelectorAll<HTMLButtonElement>(".demo-controls button").forEach((button) => { button.disabled = false; });
}

document.querySelector("#run-pass")?.addEventListener("click", () => runDemo("pass"));
document.querySelector("#run-fail")?.addEventListener("click", () => runDemo("fail"));
document.querySelector("#reset-demo")?.addEventListener("click", () => runDemo("pass"));
