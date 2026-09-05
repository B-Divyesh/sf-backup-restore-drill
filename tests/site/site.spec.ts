import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { existsSync, statSync } from "node:fs";
import { resolve } from "node:path";
import { expect, test } from "@playwright/test";

function runCliTest(name: string) {
  const output = execFileSync("cargo", ["test", "--test", "cli", name, "--", "exact"], {
    cwd: resolve("."),
    encoding: "utf8"
  });
  expect(output).toContain("test result: ok");
}

test("home is semantic, responsive, and free of serious accessibility issues", async ({ page }) => {
  const errors: string[] = [];
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  await page.goto("/");
  await expect(page).toHaveTitle("Restore Drill — Check a backup file restores");
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await expect(page.locator("main")).toHaveCount(1);
  await expect(page.locator("h1")).toHaveCount(1);
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations.filter((item) => ["serious", "critical"].includes(item.impact ?? ""))).toEqual([]);
  expect(errors).toEqual([]);
});

test("keyboard controls, focus rings, and reduced motion work", async ({ page }) => {
  await page.goto("/demo/");
  const rerun = page.getByRole("button", { name: "Run sample restore again" });
  await rerun.focus();
  await page.keyboard.press("Tab");
  await page.keyboard.press("Shift+Tab");
  await expect(rerun).toBeFocused();
  const outline = await rerun.evaluate((element) => getComputedStyle(element).outline);
  expect(outline).toContain("rgb(164, 97, 0)");
  await page.keyboard.press("Enter");
  await expect(page.locator("#stamp")).toHaveText("Verified");
  await page.getByRole("button", { name: "Show missing-file result" }).focus();
  await page.keyboard.press("Space");
  await expect(page.getByText("Missing file detected", { exact: true })).toBeVisible();

  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");
  const motion = await page.evaluate(() => ({
    animation: getComputedStyle(document.querySelector(".settle")!).animationDuration,
    transition: getComputedStyle(document.querySelector(".button")!).transitionDuration,
    scrolling: getComputedStyle(document.documentElement).scrollBehavior
  }));
  expect(motion.animation).toMatch(/0\.01ms|1e-05s/);
  expect(motion.transition).toMatch(/0\.01ms|1e-05s/);
  expect(motion.scrolling).toBe("auto");
});

test("landing shows all three facts before scrolling", async ({ page }) => {
  await page.goto("/");
  expect(await page.evaluate(() => scrollY)).toBe(0);
  for (const text of ["Free and MIT licensed.", "No usage tracking.", "Works offline after the first visit."]) {
    const box = await page.getByText(text, { exact: true }).boundingBox();
    expect(box?.y ?? Infinity).toBeLessThan(844);
    expect((box?.y ?? Infinity) + (box?.height ?? Infinity)).toBeLessThanOrEqual(844);
  }
});

test("one landing click shows completed sample output and receipt in the phone viewport", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: "Try it with sample data" }).first().click();
  await expect(page).toHaveURL(/\/demo\//);
  for (const text of ["All checks passed", "Documents/quarterly-tax-notes.txt", "SHA-256", "match", "Cleanup", "complete", "Receipt", "hash-linked JSON", "0 / passed"]) {
    const box = await page.getByText(text, { exact: true }).boundingBox();
    expect(box?.y ?? Infinity).toBeLessThan(844);
    expect((box?.y ?? Infinity) + (box?.height ?? Infinity)).toBeLessThanOrEqual(844);
  }
  await expect(page.locator('img[src="/restore-drill-demo-recording.svg"]')).toHaveAttribute("alt", /Terminal recording/);
});

test("@claim:demo-isolation bundled CLI demo leaves unrelated data unchanged", async ({ page }) => {
  runCliTest("bundled_demo_leaves_unrelated_working_directory_data_unchanged");
  await page.goto("/demo/");
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.getByRole("heading", { name: "All checks passed" })).toBeVisible();
  await expect(page.getByRole("link", { name: "Start for real" })).toBeVisible();
});

test("@claim:restore-verification selected files pass, corrupt files fail, and missing files fail", () => {
  runCliTest("selected_file_verification_handles_pass_hash_mismatch_and_missing_file");
});

test("@claim:configured-command configured restore arguments reach the configured command", () => {
  runCliTest("configured_command_receives_the_configured_arguments_without_shell_expansion");
});

test("@claim:temporary-folder-lifecycle every result removes its temporary restore folder before writing a receipt", () => {
  runCliTest("temporary_folder_lifecycle_cleans_before_every_receipt");
});

test("@claim:application-check a failed application check fails the drill", () => {
  runCliTest("application_check_controls_the_result");
});

test("@claim:path-safety unsafe paths and links outside the temporary restore folder are rejected", () => {
  runCliTest("path_safety_rejects_unsafe_configuration_and_restored_links");
});

test("@claim:receipt-integrity receipts are read-only JSON, hash linked, and tampering is detected", () => {
  runCliTest("receipt_files_are_read_only_json_linked_and_tamper_detected");
});

test("@claim:receipt-redaction receipts exclude backup-command output and restored contents", () => {
  runCliTest("receipts_exclude_restore_output_arguments_and_restored_bytes");
});

test("@claim:json-and-exit-codes JSON modes parse and documented exit states occur", () => {
  runCliTest("json_modes_and_exit_codes_cover_current_failed_overdue_and_never_run");
});

test("@claim:browser-privacy demo uses no forms, browser storage, cookies, or off-site requests", async ({ page }) => {
  const requests: string[] = [];
  page.on("request", (request) => requests.push(request.url()));
  await page.goto("/demo/");
  await page.getByRole("button", { name: "Run sample restore again" }).click();
  await expect(page.locator("#stamp")).toHaveText("Verified");
  expect(await page.locator("form").count()).toBe(0);
  expect(await page.evaluate(() => ({
    cookies: document.cookie,
    local: localStorage.length,
    session: sessionStorage.length
  }))).toEqual({ cookies: "", local: 0, session: 0 });
  expect(requests.every((url) => new URL(url).origin === "http://127.0.0.1:4173")).toBeTruthy();
});

test("@claim:offline-reload a first-visited demo reloads offline", async ({ browser }) => {
  const context = await browser.newContext();
  const page = await context.newPage();
  try {
    await page.goto("http://127.0.0.1:4173/demo/");
    await page.evaluate(() => navigator.serviceWorker.ready);
    await page.reload();
    await context.setOffline(true);
    await page.reload();
    await expect(page.locator("#offline")).toBeVisible();
    await expect(page.getByRole("heading", { name: "View a completed sample restore" })).toBeVisible();
  } finally {
    await context.close();
  }
});

test("@claim:license the packaged crate carries its MIT license", ({}, testInfo) => {
  test.skip(testInfo.project.name !== "desktop-chromium", "one packaged-artifact observation is enough");
  execFileSync("cargo", ["package", "--allow-dirty"], { cwd: resolve("."), encoding: "utf8" });
  const crate = resolve("target/package/restore-drill-0.1.0.crate");
  const contents = execFileSync("tar", ["-tzf", crate], { encoding: "utf8" });
  expect(contents).toContain("restore-drill-0.1.0/LICENSE");
  const license = execFileSync("tar", ["-xOzf", crate, "restore-drill-0.1.0/LICENSE"], { encoding: "utf8" });
  expect(license).toContain("Permission is hereby granted, free of charge");
});

test("@claim:build-artifacts build creates the documented command and site folders", ({}, testInfo) => {
  test.skip(testInfo.project.name !== "desktop-chromium", "one build observation is enough");
  execFileSync("npm", ["run", "build"], { cwd: resolve("."), encoding: "utf8" });
  expect(existsSync(resolve("dist/bin/restore-drill"))).toBeTruthy();
  expect(statSync(resolve("dist/bin/restore-drill")).size).toBeGreaterThan(0);
  expect(existsSync(resolve("dist/site/index.html"))).toBeTruthy();
  expect(existsSync(resolve("dist/site/staticwebapp.config.json"))).toBeTruthy();
});

test("@claim:response-policy built site carries the documented security and cache settings", ({}, testInfo) => {
  test.skip(testInfo.project.name !== "desktop-chromium", "one policy observation is enough");
  const output = execFileSync("npm", ["run", "test:response-policy"], { cwd: resolve("."), encoding: "utf8" });
  expect(output).toContain("response-policy:");
});

test("routes have metadata, focusable headings, and a designed 404", async ({ page }) => {
  for (const [path, title] of [["/demo/", "Demo — Restore Drill"], ["/privacy/", "Privacy — Restore Drill"], ["/terms/", "Terms — Restore Drill"]]) {
    await page.goto(path);
    await expect(page).toHaveTitle(title);
    await expect(page.locator('link[rel="canonical"]')).toHaveCount(1);
    await expect(page.locator('meta[property="og:image"]')).toHaveCount(1);
    await expect(page.locator("h1")).toHaveCount(1);
    await expect(page.getByLabel("Primary navigation").getByRole("link")).toHaveCount(4);
    await expect(page.getByLabel("Footer links").getByRole("link")).toHaveCount(4);
  }
  const response = await page.goto("/does-not-exist-repair-2");
  expect(response?.status()).toBe(404);
  await expect(page).toHaveTitle("Page not found — Restore Drill");
  await expect(page.locator('link[rel="canonical"]')).toHaveCount(1);
  await expect(page.locator('meta[property="og:image"]')).toHaveCount(1);
  await expect(page.locator('meta[name="twitter:card"]')).toHaveCount(1);
  await expect(page.getByRole("link", { name: "Return to Restore Drill" })).toBeVisible();
});

test("direct, cross-page, and Back hash routes focus and announce their section", async ({ page }) => {
  await page.goto("/#setup");
  await expect(page.locator("#setup-title")).toBeFocused();
  await expect(page.locator("#route-status")).toHaveText("Start with one important file");

  await page.goto("/demo/");
  await page.getByRole("link", { name: "Start for real" }).click();
  await expect(page).toHaveURL(/\/#setup$/);
  await expect(page.locator("#setup-title")).toBeFocused();
  await expect(page.locator("#route-status")).toHaveText("Start with one important file");

  await page.goto("/#how-it-works");
  await expect(page.locator("#how-title")).toBeFocused();
  await page.getByLabel("Primary navigation").getByRole("link", { name: "Privacy" }).click();
  await page.goBack();
  await expect(page).toHaveURL(/\/#how-it-works$/);
  await expect(page.locator("#how-title")).toBeFocused();
  await expect(page.locator("#route-status")).toHaveText("Check one selected backup file");
});
