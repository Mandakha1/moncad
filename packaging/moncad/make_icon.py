#!/usr/bin/env python3
"""Draw the MonCad app icon (original artwork, MIT OR Apache-2.0) with Pillow.

    python3 packaging/moncad/make_icon.py   # writes assets/app-icon/cadcraft-1024.png and cadcraft-macos-512.png
    packaging/icons.sh                      # then derives every other size, the .ico and the .icns

A white drafted "M" on a deep blue sheet with a grid, CAD grips at its vertices (Mongolian-flag red)
and a dimension line underneath. The file names stay the upstream ones so packaging needs no changes.
"""
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets" / "app-icon"
S = 4096  # supersampled canvas
BLUE, BLUE2, GRID = (13, 59, 110), (8, 41, 82), (52, 104, 160)
WHITE, RED, GOLD = (248, 250, 252), (218, 41, 28), (255, 196, 46)


def draw(size: int, margin: float) -> Image.Image:
    img = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    m = int(S * margin)
    r = int((S - 2 * m) * 0.22)
    # sheet: vertical gradient
    sheet = Image.new("RGBA", (S, S))
    sd = ImageDraw.Draw(sheet)
    for y in range(S):
        t = y / S
        sd.line([(0, y), (S, y)], fill=tuple(int(BLUE[i] * (1 - t) + BLUE2[i] * t) for i in range(3)) + (255,))
    mask = Image.new("L", (S, S), 0)
    ImageDraw.Draw(mask).rounded_rectangle([m, m, S - m, S - m], r, fill=255)
    img.paste(sheet, (0, 0), mask)
    # grid
    inner = S - 2 * m
    step = inner / 12
    g = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    gd = ImageDraw.Draw(g)
    for i in range(1, 12):
        x = m + i * step
        gd.line([(x, m), (x, S - m)], fill=GRID + (110,), width=max(4, S // 400))
        gd.line([(m, x), (S - m, x)], fill=GRID + (110,), width=max(4, S // 400))
    gm = Image.new("L", (S, S), 0)
    ImageDraw.Draw(gm).rounded_rectangle([m, m, S - m, S - m], r, fill=255)
    img.paste(g, (0, 0), Image.composite(g.getchannel("A"), Image.new("L", (S, S), 0), gm))
    d = ImageDraw.Draw(img)

    def P(u, v):  # unit square inside the sheet -> pixels
        return (m + u * inner, m + v * inner)

    # the drafted M
    pts = [P(0.22, 0.70), P(0.22, 0.24), P(0.50, 0.52), P(0.78, 0.24), P(0.78, 0.70)]
    w = int(inner * 0.078)
    d.line(pts, fill=WHITE, width=w, joint="curve")
    for p in (pts[0], pts[-1]):
        d.ellipse([p[0] - w / 2, p[1] - w / 2, p[0] + w / 2, p[1] + w / 2], fill=WHITE)
    # grips (squares) at the vertices
    gsz = inner * 0.036
    for p in pts:
        d.rectangle([p[0] - gsz, p[1] - gsz, p[0] + gsz, p[1] + gsz], fill=RED, outline=WHITE, width=int(inner * 0.009))
    # dimension line under the M
    y = m + 0.83 * inner
    x0, x1 = m + 0.22 * inner, m + 0.78 * inner
    lw = int(inner * 0.018)
    d.line([(x0, y), (x1, y)], fill=GOLD, width=lw)
    for x in (x0, x1):
        d.line([(x, y - inner * 0.07), (x, y + inner * 0.03)], fill=GOLD, width=lw)
    a = inner * 0.045
    d.polygon([(x0, y), (x0 + a * 1.6, y - a * 0.55), (x0 + a * 1.6, y + a * 0.55)], fill=GOLD)
    d.polygon([(x1, y), (x1 - a * 1.6, y - a * 0.55), (x1 - a * 1.6, y + a * 0.55)], fill=GOLD)
    return img.resize((size, size), Image.LANCZOS)


if __name__ == "__main__":
    draw(1024, 0.0).save(OUT / "cadcraft-1024.png")
    # macOS grid: the artwork sits inside ~10% margins like other macOS app icons.
    draw(512, 0.098).save(OUT / "cadcraft-macos-512.png")
    print("wrote", OUT / "cadcraft-1024.png", "and cadcraft-macos-512.png")
