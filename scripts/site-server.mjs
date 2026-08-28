import { createReadStream, existsSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { extname, join, normalize, resolve } from "node:path";

const root = resolve("dist/site");
const types = { ".css": "text/css", ".html": "text/html", ".js": "text/javascript", ".jpg": "image/jpeg", ".png": "image/png", ".svg": "image/svg+xml", ".webp": "image/webp", ".xml": "application/xml", ".txt": "text/plain" };
createServer((request, response) => {
  const pathname = decodeURIComponent(new URL(request.url ?? "/", "http://localhost").pathname);
  const safe = normalize(pathname).replace(/^([/\\])+/, "");
  let file = join(root, safe || "index.html");
  if (existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
  const found = existsSync(file) && statSync(file).isFile();
  if (!found) file = join(root, "404.html");
  response.writeHead(found ? 200 : 404, { "content-type": `${types[extname(file)] ?? "application/octet-stream"}; charset=utf-8` });
  createReadStream(file).pipe(response);
}).listen(4173, "127.0.0.1");
