import AxeBuilder from "@axe-core/playwright";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { expect, test } from "@playwright/test";

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

test("@claim:demo-isolation bundled CLI demo is disposable and browser demo has a reset banner", async ({ page }) => {
  const output = execFileSync("cargo", ["run", "--quiet", "--", "demo", "--json"], { cwd: resolve("."), encoding: "utf8" });
  expect(JSON.parse(output)).toMatchObject({ status: "pass", receipt_count: 1, workspace: "removed" });
  expect(existsSync(resolve(".restore-drill"))).toBeFalsy();
  await page.goto("/demo/");
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
  await expect(page.getByRole("button", { name: "Reset demo" })).toBeVisible();
  await expect(page.getByRole("link", { name: "Start for real" })).toBeVisible();
});

test("@claim:restore-verification demo proves the shipped sample against its fingerprint", async ({ page }) => {
  await page.goto("/demo/");
  await expect(page.getByRole("heading", { name: "All checks passed" })).toBeVisible();
  await expect(page.locator("#receipt-hash")).toContainText("match");
  await page.getByRole("button", { name: "Show missing-file result" }).click();
  await expect(page.getByText("Missing file detected", { exact: true })).toBeVisible();
  await expect(page.locator("#terminal")).toContainText("exit 1");
});

test("@claim:local-network demo makes only same-origin browser requests", async ({ page }) => {
  const urls: string[] = [];
  page.on("request", (request) => urls.push(request.url()));
  await page.goto("/demo/");
  await page.getByRole("button", { name: "Run sample restore again" }).click();
  await expect(page.locator("#stamp")).toHaveText("Verified");
  expect(urls.every((url) => new URL(url).origin === "http://127.0.0.1:4173")).toBeTruthy();
});

test("@claim:offline-reload home and demo reload after the first visit", async ({ page, context }) => {
  await page.goto("/demo/");
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload();
  await context.setOffline(true);
  await page.reload();
  await expect(page.locator("#offline")).toBeVisible();
  await expect(page.getByRole("heading", { name: "See one backup file restore" })).toBeVisible();
});

test("@claim:license repository ships the MIT license without a paid gate", () => {
  expect(readFileSync(resolve("LICENSE"), "utf8")).toContain("Permission is hereby granted, free of charge");
  expect(readFileSync(resolve("Cargo.toml"), "utf8")).toContain('license = "MIT"');
});

test("@claim:receipt-integrity changed hash-linked receipts are detected", () => {
  const output = execFileSync("cargo", ["test", "detects_missing_sample_and_receipt_tampering", "--", "--exact"], { cwd: resolve("."), encoding: "utf8" });
  expect(output).toContain("test result: ok");
});

test("@claim:path-safety unsafe sample paths and a missing restore placeholder are rejected", () => {
  const output = execFileSync("cargo", ["test", "rejects_unsafe_sample_paths_and_missing_target", "--", "--exact"], { cwd: resolve("."), encoding: "utf8" });
  expect(output).toContain("test result: ok");
});

test("routes have metadata, focusable headings, and a designed 404", async ({ page }) => {
  for (const [path, title] of [["/demo/", "Demo — Restore Drill"], ["/privacy/", "Privacy — Restore Drill"], ["/terms/", "Terms — Restore Drill"]]) {
    await page.goto(path);
    await expect(page).toHaveTitle(title);
    await expect(page.locator('link[rel="canonical"]')).toHaveCount(1);
    await expect(page.locator('meta[property="og:image"]')).toHaveCount(1);
    await expect(page.locator("h1")).toHaveCount(1);
  }
  const response = await page.goto("/does-not-exist-review-1");
  expect(response?.status()).toBe(404);
  await expect(page).toHaveTitle("Page not found — Restore Drill");
  await expect(page.getByRole("link", { name: "Return to Restore Drill" })).toBeVisible();
});

test("query demo route redirects and keyboard section navigation moves focus", async ({ page }) => {
  await page.goto("/?demo=1");
  await expect(page).toHaveURL(/\/demo\/\?demo=1/);
  await page.goto("/");
  await page.getByRole("link", { name: "How it works" }).focus();
  await page.keyboard.press("Enter");
  await expect(page.locator("#how-title")).toBeFocused();
});
