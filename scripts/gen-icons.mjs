/**
 * Generates all Tauri-required icon files in src-tauri/icons/
 * using only Node.js built-ins (no canvas dependency).
 *
 * Icon design: dark background with a stylised ✦ sparkle mark.
 */

import { createWriteStream, mkdirSync } from "fs";
import { deflateSync, crc32 } from "zlib";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const OUT = join(__dirname, "../src-tauri/icons");
mkdirSync(OUT, { recursive: true });

// ── PNG encoder ──────────────────────────────────────────────────────────────

function crc(buf) {
  // Node's zlib.crc32 signature: crc32(data, crc?) → number
  return crc32(buf);
}

function u32be(n) {
  const b = Buffer.alloc(4);
  b.writeUInt32BE(n, 0);
  return b;
}

function chunk(type, data) {
  const t = Buffer.from(type, "ascii");
  const d = Buffer.isBuffer(data) ? data : Buffer.from(data);
  const len = u32be(d.length);
  const crcVal = u32be(crc32(Buffer.concat([t, d])));
  return Buffer.concat([len, t, d, crcVal]);
}

function encodePNG(pixels, w, h) {
  // pixels: Uint8Array of RGBA, row-major
  const sig = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(w, 0);
  ihdr.writeUInt32BE(h, 4);
  ihdr[8] = 8;  // bit depth
  ihdr[9] = 2;  // colour type: RGB  (we'll strip alpha for simplicity → use 6 for RGBA)
  ihdr[9] = 6;  // RGBA
  ihdr[10] = 0; // compression
  ihdr[11] = 0; // filter
  ihdr[12] = 0; // interlace

  // Build raw scanlines with filter byte 0 (None) prepended
  const rowSize = w * 4;
  const raw = Buffer.alloc(h * (rowSize + 1));
  for (let y = 0; y < h; y++) {
    raw[y * (rowSize + 1)] = 0; // filter None
    pixels.copy(raw, y * (rowSize + 1) + 1, y * rowSize, (y + 1) * rowSize);
  }

  const compressed = deflateSync(raw, { level: 6 });

  return Buffer.concat([
    sig,
    chunk("IHDR", ihdr),
    chunk("IDAT", compressed),
    chunk("IEND", Buffer.alloc(0)),
  ]);
}

// ── Icon renderer ────────────────────────────────────────────────────────────

function renderIcon(size) {
  const pixels = Buffer.alloc(size * size * 4);

  const cx = size / 2;
  const cy = size / 2;
  const r = size / 2;

  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const idx = (y * size + x) * 4;
      const dx = x - cx;
      const dy = y - cy;
      const dist = Math.sqrt(dx * dx + dy * dy);

      // Background: deep navy circle, corners transparent
      if (dist > r) {
        pixels[idx + 3] = 0; // transparent
        continue;
      }

      // Base colour: #1a1d27 (dark surface)
      let bgR = 26, bgG = 29, bgB = 39;

      // Subtle radial gradient → slightly lighter at centre
      const t = 1 - dist / r;
      bgR = Math.round(bgR + t * 12);
      bgG = Math.round(bgG + t * 12);
      bgB = Math.round(bgB + t * 20);

      // ✦ sparkle: four diamond lobes in primary blue #4f8ef7
      const angle = Math.atan2(dy, dx);
      const norm = dist / r;

      // Four-pointed star: r(θ) = cos²(2θ) shape
      const starR = Math.pow(Math.abs(Math.cos(2 * angle)), 0.5) * r * 0.55;
      const inStar = dist < starR * 0.92;
      const starEdge = dist >= starR * 0.92 && dist < starR;

      // Inner bright spot
      const innerSpot = dist < r * 0.12;

      let fR = bgR, fG = bgG, fB = bgB, fA = 255;

      if (inStar) {
        // Primary blue #4f8ef7
        fR = 79; fG = 142; fB = 247;
        // Lighten towards centre of each lobe
        const lobe = 1 - dist / (starR * 0.92);
        fR = Math.min(255, Math.round(fR + lobe * 60));
        fG = Math.min(255, Math.round(fG + lobe * 40));
        fB = Math.min(255, Math.round(fB + lobe * 8));
      } else if (starEdge) {
        // Anti-alias blend
        const blend = (dist - starR * 0.92) / (starR * 0.08);
        fR = Math.round(79 + blend * (bgR - 79));
        fG = Math.round(142 + blend * (bgG - 142));
        fB = Math.round(247 + blend * (bgB - 247));
      }

      if (innerSpot) {
        fR = Math.min(255, fR + 80);
        fG = Math.min(255, fG + 80);
        fB = Math.min(255, fB + 80);
      }

      // Soft edge of circle
      if (dist > r * 0.96) {
        const edgeBlend = (dist - r * 0.96) / (r * 0.04);
        fA = Math.round(255 * (1 - edgeBlend));
      }

      pixels[idx] = fR;
      pixels[idx + 1] = fG;
      pixels[idx + 2] = fB;
      pixels[idx + 3] = fA;
    }
  }

  return encodePNG(pixels, size, size);
}

// ── Write PNGs ────────────────────────────────────────────────────────────────

const pngSizes = [
  { name: "32x32.png",      size: 32 },
  { name: "128x128.png",    size: 128 },
  { name: "128x128@2x.png", size: 256 },
  { name: "icon.png",       size: 512 },
];

for (const { name, size } of pngSizes) {
  const buf = renderIcon(size);
  const dest = join(OUT, name);
  const ws = createWriteStream(dest);
  ws.write(buf);
  ws.end();
  console.log(`  wrote ${dest} (${size}×${size}, ${buf.length} bytes)`);
}

// ── Write .ico (multi-size: 16, 32, 48, 256) ─────────────────────────────────

function buildIco(sizes) {
  // ICO format: header + directory entries + image data
  const images = sizes.map((s) => {
    const png = renderIcon(s);
    return { size: s, data: png };
  });

  const headerSize = 6;
  const dirEntrySize = 16;
  const dirSize = dirEntrySize * images.length;
  const dataOffset = headerSize + dirSize;

  let offset = dataOffset;
  const entries = images.map(({ size, data }) => {
    const entry = Buffer.alloc(16);
    entry[0] = size >= 256 ? 0 : size; // width (0 = 256)
    entry[1] = size >= 256 ? 0 : size; // height
    entry[2] = 0; // colour count
    entry[3] = 0; // reserved
    entry.writeUInt16LE(1, 4);  // planes
    entry.writeUInt16LE(32, 6); // bit count
    entry.writeUInt32LE(data.length, 8);
    entry.writeUInt32LE(offset, 12);
    offset += data.length;
    return entry;
  });

  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0); // reserved
  header.writeUInt16LE(1, 2); // type: ICO
  header.writeUInt16LE(images.length, 4);

  return Buffer.concat([header, ...entries, ...images.map((i) => i.data)]);
}

const ico = buildIco([16, 32, 48, 256]);
import { writeFileSync } from "fs";
writeFileSync(join(OUT, "icon.ico"), ico);
console.log(`  wrote ${join(OUT, "icon.ico")} (${ico.length} bytes)`);

// ── Write .icns (Apple Icon Image) ───────────────────────────────────────────
// icns format: 'icns' magic + total length, then type+length+data chunks

function buildIcns(entries) {
  // entries: [{type: 'ic07', size: 128}, ...]  — PNG data embedded directly
  const chunks = entries.map(({ type, imgSize }) => {
    const png = renderIcon(imgSize);
    const header = Buffer.alloc(8);
    header.write(type, 0, "ascii");
    header.writeUInt32BE(8 + png.length, 4);
    return Buffer.concat([header, png]);
  });

  const body = Buffer.concat(chunks);
  const header = Buffer.alloc(8);
  header.write("icns", 0, "ascii");
  header.writeUInt32BE(8 + body.length, 4);
  return Buffer.concat([header, body]);
}

const icns = buildIcns([
  { type: "ic07", imgSize: 128 },  // 128×128
  { type: "ic08", imgSize: 256 },  // 256×256
  { type: "ic09", imgSize: 512 },  // 512×512
  { type: "ic10", imgSize: 1024 }, // 1024×1024
]);
writeFileSync(join(OUT, "icon.icns"), icns);
console.log(`  wrote ${join(OUT, "icon.icns")} (${icns.length} bytes)`);

console.log("\nAll icons generated successfully.");
