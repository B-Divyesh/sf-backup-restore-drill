import { copyFileSync, mkdirSync } from "node:fs";
import { platform } from "node:process";

mkdirSync("dist/bin", { recursive: true });
const suffix = platform === "win32" ? ".exe" : "";
copyFileSync(`target/release/restore-drill${suffix}`, `dist/bin/restore-drill${suffix}`);
