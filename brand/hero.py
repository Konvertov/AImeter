# Three characters of our own in the spirit of the reference: a blocky
# creature in sunglasses. The silhouettes are ours; each one carries a
# limit bar somewhere, because that is what the app is about.
ORANGE, DARK, CREAM, DEEP = "#D97757", "#1b1b1f", "#F6F0E4", "#B5573C"

def outlined(pieces, fill=ORANGE, edge=12):
    """Several shapes that read as one body: outline first, then fill on top."""
    under = "".join(p.replace("/>", f' fill="{DARK}" stroke="{DARK}" stroke-width="{edge * 2}" stroke-linejoin="round"/>') for p in pieces)
    over = "".join(p.replace("/>", f' fill="{fill}"/>') for p in pieces)
    return under + over

def glasses(cx, cy, w):
    h = 0.25 * w
    def lens(side):
        pts = [(side * 0.5 * w, 0), (side * 0.045 * w, 0), (side * 0.085 * w, h), (side * 0.42 * w, h)]
        return " ".join(f"{cx + x:.0f},{cy + y:.0f}" for x, y in pts)
    def glint(side):
        x0 = cx + side * 0.27 * w - 0.02 * w
        return (f'<path d="M{x0:.0f} {cy + 0.19 * h + 0.5 * h:.0f} L{x0 + 0.07 * w:.0f} {cy + 0.2 * h:.0f}" stroke="#ffffff" stroke-width="{0.022 * w:.0f}" stroke-linecap="round"/>'
                f'<path d="M{x0 + 0.05 * w:.0f} {cy + 0.19 * h + 0.5 * h:.0f} L{x0 + 0.1 * w:.0f} {cy + 0.32 * h:.0f}" stroke="#ffffff" stroke-width="{0.012 * w:.0f}" stroke-linecap="round" opacity=".8"/>')
    return (f'<rect x="{cx - 0.5 * w:.0f}" y="{cy - 0.03 * w:.0f}" width="{w:.0f}" height="{0.06 * w:.0f}" rx="{0.015 * w:.0f}" fill="{DARK}"/>'
            f'<polygon points="{lens(-1)}" fill="{DARK}" stroke="{DARK}" stroke-width="{0.03 * w:.0f}" stroke-linejoin="round"/>'
            f'<polygon points="{lens(1)}" fill="{DARK}" stroke="{DARK}" stroke-width="{0.03 * w:.0f}" stroke-linejoin="round"/>'
            + glint(-1) + glint(1))

def smile(x1, y, x2):
    return f'<path d="M{x1} {y} Q{(x1 + x2) / 2:.0f} {y + 46} {x2} {y - 8}" fill="none" stroke="{DARK}" stroke-width="15" stroke-linecap="round"/>'

def shadow(cx, cy, rx):
    return f'<ellipse cx="{cx}" cy="{cy}" rx="{rx}" ry="20" fill="#000000" opacity=".13"/>'

def block():
    """A square fellow with one antenna and a row of charge cells on the belly."""
    body = ['<rect x="292" y="330" width="440" height="380" rx="10"/>',
            '<rect x="492" y="262" width="40" height="72"/>', '<rect x="468" y="214" width="88" height="54" rx="6"/>',
            '<rect x="352" y="706" width="72" height="92"/>', '<rect x="600" y="706" width="72" height="92"/>',
            '<rect x="244" y="548" width="52" height="74"/>', '<rect x="728" y="548" width="52" height="74"/>']
    cells = "".join(f'<rect x="{372 + i * 74}" y="638" width="58" height="32" rx="6" fill="{CREAM if i < 3 else DEEP}"/>' for i in range(4))
    return shadow(512, 804, 220) + outlined(body) + glasses(512, 432, 400) + smile(446, 578, 590) + cells

def screen():
    """A little monitor on a stand; the bar under its smile is the limit."""
    body = ['<rect x="226" y="296" width="572" height="372" rx="26"/>',
            '<rect x="480" y="664" width="64" height="70"/>', '<rect x="384" y="730" width="256" height="48" rx="10"/>',
            '<rect x="300" y="262" width="60" height="38" rx="6"/>', '<rect x="664" y="262" width="60" height="38" rx="6"/>']
    bar = (f'<rect x="336" y="576" width="352" height="38" rx="19" fill="{DEEP}"/>'
           f'<rect x="336" y="576" width="218" height="38" rx="19" fill="{CREAM}"/>')
    return shadow(512, 786, 200) + outlined(body) + glasses(512, 398, 430) + smile(440, 528, 596) + bar

def bubble():
    """Our chat-bubble logo come alive: same outline, now with a face."""
    body = ['<path d="M312 222 H712 a130 130 0 0 1 130 130 V562 a130 130 0 0 1 -130 130 H506 L352 832 V692 H312 a130 130 0 0 1 -130 -130 V352 a130 130 0 0 1 130 -130 Z"/>']
    bar = (f'<rect x="332" y="590" width="360" height="40" rx="20" fill="{DEEP}"/>'
           f'<rect x="332" y="590" width="223" height="40" rx="20" fill="{CREAM}"/>')
    return shadow(470, 846, 170) + outlined(body) + glasses(512, 372, 450) + smile(438, 508, 594) + bar

IDEAS = [
    ("1. Квадрик", block, "Квадратный персонаж с одной антенной. На животе — четыре ячейки заряда: три горят, одна потрачена."),
    ("2. Экранчик", screen, "Маленький монитор на подставке. Под улыбкой — полоска лимита."),
    ("3. Облачко", bubble, "Наш нынешний логотип-облачко, только ожившее: тот же контур, очки, улыбка и полоска."),
]

def round_badge(draw, size):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="{size}" height="{size}">'
            f'<circle cx="512" cy="512" r="512" fill="{CREAM}"/><g transform="translate(512 512) scale(1.08) translate(-512 -508)">{draw()}</g></svg>')

def tile(draw, size, bg="#14161c"):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="{size}" height="{size}">'
            f'<rect width="1024" height="1024" rx="230" fill="{bg}"/><g transform="translate(512 512) scale(1.16) translate(-512 -508)">{draw()}</g></svg>')

cards = ""
for index, (name, draw, about) in enumerate(IDEAS):
    open(f"hero-{index + 1}.svg", "w").write(round_badge(draw, 1024))
    cards += f"""
<article>
  <h2>{name}</h2>
  <div class="row">{round_badge(draw, 260)}{tile(draw, 200)}{tile(draw, 200, CREAM)}</div>
  <div class="row small">{tile(draw, 96)}{tile(draw, 64)}{tile(draw, 32)}{tile(draw, 16)}
    <div class="lock">{tile(draw, 56, CREAM)}<span><b>AI</b> Meter</span></div></div>
  <p>{about}</p>
</article>"""

page = f"""<!doctype html><html lang="ru"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>AI Meter — персонаж в очках</title><style>
body{{margin:0;background:#eef0f3;color:#14161c;font:15px/1.5 -apple-system,"Segoe UI",system-ui,sans-serif}}
main{{max-width:1180px;margin:0 auto;padding:36px 20px 60px}}
h1{{margin:0 0 6px;font-size:30px;letter-spacing:-.02em}} h1 span{{color:{ORANGE}}}
.lead{{margin:0 0 24px;color:#586170;max-width:60em}}
article{{background:#fff;border:1px solid #dfe3e8;border-radius:18px;padding:22px;margin-bottom:18px}}
h2{{margin:0 0 14px;font-size:19px}} p{{margin:14px 0 0;color:#586170}}
.row{{display:flex;flex-wrap:wrap;gap:18px;align-items:flex-end;margin-top:14px}}
.row>svg{{box-shadow:0 8px 22px rgba(20,22,28,.15);border-radius:22.5%}} .row>svg:first-child{{border-radius:50%}}
.small>svg:first-child{{border-radius:22.5%}}
.lock{{display:flex;align-items:center;gap:12px;background:#14161c;color:#f2f4f8;border-radius:14px;padding:12px 20px;font-size:30px;font-weight:700;letter-spacing:-.02em}}
.lock svg{{border-radius:22.5%}} .lock b{{color:{ORANGE}}}
</style></head><body><main>
<h1><span>AI</span> Meter — персонаж в очках</h1>
<p class="lead">Три собственных персонажа в духе присланной картинки: угловатый герой в тёмных очках, с улыбкой и тенью. Силуэты нарисованы свои, и у каждого где-то на теле есть полоска или ячейки лимита.</p>
{cards}
</main></body></html>"""
open("hero.html", "w").write(page)
print("ok")
