#!/usr/bin/env python3
"""Recolour VKB's reference sheets to astromech's dark palette (docs/V1-DESIGN.md §3.2).

For each PDF in data/devices/vkb/:
  1. Render page 1 at a fixed 150 DPI (1 PDF point = 150/72 px), so form-field boxes from the
     device maps still line up: the app scales by page points, not pixels.
  2. Find the stick photo with rembg (isnet-general-use, CPU, a few seconds per sheet); keep the
     largest connected blob as the mask.
  3. Everything outside the mask is flat artwork. Split each pixel into a local background level
     (grey dilation) and "ink" on top of it, then map:
        white page   -> --bg        light fields (both stripe shades) -> --cell-empty (unbound box)
        grey header  -> --cell-opt  black ink    -> light grey text/lines
        red leaders  -> muted orange (--accent-dim)  blue link -> grey
  4. Tone-map the photo for a dark background, and draw a 2 px outline round the mask.
Writes app/public/devices/vkb/<name>.png (the files the app shows). Credit: VKB Gaming,
sheet design by UntoldForce. Needs numpy, pillow, opencv-python-headless, rembg.
"""
import glob, os, subprocess, tempfile
import cv2
import numpy as np
from PIL import Image
from rembg import new_session, remove

DPI = 150
BG = np.array([12, 13, 15]); OPT = np.array([43, 45, 46])
CELL = np.array([28, 22, 16])  # --cell-empty
INK = np.array([215, 215, 215]); RED = np.array([150, 70, 10])
OUTLINE = np.array([160, 160, 160])

def render(pdf):
    out = tempfile.mktemp()
    subprocess.run(["pdftoppm", "-png", "-singlefile", "-r", str(DPI), pdf, out], check=True)
    return np.array(Image.open(out + ".png").convert("RGB"))

def stick_mask(rgb, session):
    m = np.array(remove(Image.fromarray(rgb), session=session, only_mask=True)) > 128
    n, lab, stats, _ = cv2.connectedComponentsWithStats(m.astype(np.uint8))
    keep = 1 + int(np.argmax(stats[1:, cv2.CC_STAT_AREA]))
    m = (lab == keep).astype(np.uint8)
    m = cv2.morphologyEx(m, cv2.MORPH_CLOSE, np.ones((9, 9), np.uint8))
    return m.astype(bool)

def artwork(rgb):
    f = rgb.astype(np.float32)
    lum = f.mean(2)
    r, g, b = f[..., 0], f[..., 1], f[..., 2]
    base = cv2.dilate(lum, np.ones((7, 7), np.uint8))  # local background level
    wide = cv2.dilate(lum, np.ones((31, 31), np.uint8))  # for thick strokes (the title)
    base = np.where(base > 170, base, wide)
    # background class from the level: >240 page, 215-240 field, 198-215 mid, else header
    bgc = np.where((base > 240)[..., None], BG,
          np.where((base > 215)[..., None], CELL,
          np.where((base > 198)[..., None], CELL, OPT))).astype(np.float32)
    a = np.clip((base - lum) / np.maximum(base - 20, 1), 0, 1)[..., None]  # ink amount
    red = ((r - g) > 60) & ((r - b) > 60)
    ink = np.where(red[..., None], RED, INK).astype(np.float32)
    a = np.where(red[..., None], np.clip((r - g)[..., None] / 200, 0, 1), a)
    return bgc * (1 - a) + ink * a

def photo(rgb):
    f = rgb.astype(np.float32) / 255
    hsv = cv2.cvtColor(rgb, cv2.COLOR_RGB2HSV).astype(np.float32)
    hsv[..., 2] = 255 * (0.18 + 0.72 * (hsv[..., 2] / 255) ** 0.75)  # lift shadows
    hsv[..., 1] *= 0.85
    out = cv2.cvtColor(hsv.astype(np.uint8), cv2.COLOR_HSV2RGB).astype(np.float32)
    return out * (f.mean(2, keepdims=True) < 0.97) + BG * (f.mean(2, keepdims=True) >= 0.97)

def main():
    session = new_session("isnet-general-use")
    for pdf in sorted(glob.glob("data/devices/vkb/*.pdf")):
        rgb = render(pdf)
        m = stick_mask(rgb, session)
        soft = cv2.GaussianBlur(m.astype(np.float32), (5, 5), 0)[..., None]
        out = artwork(rgb) * (1 - soft) + photo(rgb) * soft
        ring = cv2.dilate(m.astype(np.uint8), np.ones((5, 5), np.uint8)) - m.astype(np.uint8)
        ring = cv2.GaussianBlur(ring.astype(np.float32), (3, 3), 0)[..., None]
        out = out * (1 - ring) + OUTLINE * ring
        name = os.path.splitext(os.path.basename(pdf))[0]
        os.makedirs("app/public/devices/vkb", exist_ok=True)
        Image.fromarray(np.clip(out, 0, 255).astype(np.uint8)).save(f"app/public/devices/vkb/{name}.png", optimize=True)
        print(name, rgb.shape[1], "x", rgb.shape[0], "px at", DPI, "DPI; stick", int(m.sum()), "px")

if __name__ == "__main__":
    main()
