import { existsSync, readFileSync } from "fs";
import { decode } from "./index";

const p = process.argv[2];
if (!p) {
  console.log("free-piano-midi <FILE.mid>");
  process.exit();
}

if (!existsSync(p)) {
  console.log(`${p} not found`);
  process.exit();
}

const bytes = readFileSync(p);

const json = decode(bytes)?.map((i) => {
  return { start: i.start, end: i.end, code: i.code };
});

console.log(JSON.stringify(json));
