# One logo, drawn by hand in vectors: the letter A as a vessel.
# Two thick legs make the "A"; the space between them is filled with orange
# up to a level, the way a limit is used up. The surface of the fill is the
# crossbar of the letter.
ORANGE = "#D97757"

def mark(ink, fill, dim=None, level=560):
    """The sign on a 1024 box. `level` is the y of the fill surface (lower = fuller)."""
    apex, foot_y, half = (512, 236), 772, 262
    slope = half / (foot_y - apex[1])
    inset = 93.5  # leg half-width plus a gap, measured along x
    def inner(y, side):
        return apex[0] + side * ((y - apex[1]) * slope - inset)
    bottom = 790
    body = (f"M{inner(level,-1):.1f} {level} L{inner(level,1):.1f} {level} "
            f"L{inner(bottom,1):.1f} {bottom} L{inner(bottom,-1):.1f} {bottom} Z")
    legs = f"M{apex[0]-half} {foot_y} L{apex[0]} {apex[1]} L{apex[0]+half} {foot_y}"
    out = ""
    if dim:  # the empty part of the vessel, above the surface
        top = 430
        out += (f'<path d="M{inner(top,-1):.1f} {top} L{inner(top,1):.1f} {top} '
                f'L{inner(level-44,1):.1f} {level-44} L{inner(level-44,-1):.1f} {level-44} Z" '
                f'fill="{dim}" stroke="{dim}" stroke-width="28" stroke-linejoin="round"/>')
    out += f'<path d="{body}" fill="{fill}" stroke="{fill}" stroke-width="28" stroke-linejoin="round"/>'
    out += (f'<path d="{legs}" fill="none" stroke="{ink}" stroke-width="100" '
            f'stroke-linecap="round" stroke-linejoin="round"/>')
    return out

def icon(dark=True, size=1024, **kw):
    bg, ink, dim = ("#14161c", "#f2f4f8", "#2c3140") if dark else ("#ffffff", "#14161c", "#e3e6eb")
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="{size}" height="{size}">'
            f'<rect width="1024" height="1024" rx="230" fill="{bg}"/>{mark(ink, ORANGE, dim, **kw)}</svg>')

def bare(color, size=18):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="150 150 724 724" width="{size}" height="{size}">'
            f'{mark(color, color)}</svg>')

open("ai-meter-icon.svg", "w").write(icon(True))
open("ai-meter-icon-light.svg", "w").write(icon(False))
open("ai-meter-mark.svg", "w").write(bare("currentColor", 24))

levels = "".join(
    f'<figure>{icon(True, 96, level=y)}<figcaption>{label}</figcaption></figure>'
    for y, label in [(700, "почти пусто"), (620, "треть"), (560, "половина"), (500, "почти полно")]
)
page = f"""<!doctype html><html lang="ru"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>AI Meter — логотип</title><style>
body{{margin:0;background:#eef0f3;color:#14161c;font:15px/1.5 -apple-system,"Segoe UI",system-ui,sans-serif}}
main{{max-width:980px;margin:0 auto;padding:40px 20px 60px}}
h1{{margin:0 0 6px;font-size:30px;letter-spacing:-.02em}} h1 span{{color:{ORANGE}}}
h2{{margin:34px 0 12px;font-size:18px}}
p{{margin:0 0 10px;color:#586170;max-width:46em}}
.row{{display:flex;flex-wrap:wrap;gap:22px;align-items:flex-end}}
.row svg{{border-radius:22.5%;box-shadow:0 10px 30px rgba(20,22,28,.16)}}
.bar{{display:inline-flex;align-items:center;gap:7px;padding:5px 12px;border-radius:8px;font-size:13px;font-weight:600}}
.bar svg{{box-shadow:none;border-radius:0}} .dark{{background:#23262e;color:#f2f4f8}} .light{{background:#dfe2e7;color:#14161c}}
.lock{{display:flex;align-items:center;gap:16px;background:#14161c;color:#f2f4f8;border-radius:18px;padding:22px 30px;font-size:44px;font-weight:700;letter-spacing:-.02em}}
.lock.white{{background:#fff;color:#14161c;border:1px solid #dfe3e8}} .lock b{{color:{ORANGE};font-weight:700}} .lock svg{{box-shadow:none}}
figure{{margin:0;text-align:center}} figcaption{{margin-top:8px;font-size:13px;color:#586170}}
</style></head><body><main>
<h1><span>AI</span> Meter — логотип на мой вкус</h1>
<p>Буква «A» как сосуд. Две толстые ножки — сама буква, а пространство между ними залито оранжевым до уровня, как расходуется лимит. Поверхность заливки работает как перекладина буквы.</p>
<h2>Значок приложения</h2>
<div class="row">{icon(True, 220)}{icon(False, 220)}{icon(True, 96)}{icon(True, 48)}{icon(True, 32)}{icon(True, 16)}</div>
<h2>С названием</h2>
<div class="row"><div class="lock">{icon(True, 72)}<span><b>AI</b> Meter</span></div><div class="lock white">{icon(False, 72)}<span><b>AI</b> Meter</span></div></div>
<h2>В строке меню, одним цветом</h2>
<div class="row"><span class="bar dark">{bare("#f2f4f8")}42%</span><span class="bar light">{bare("#14161c")}42%</span></div>
<h2>Уровень может быть живым</h2>
<p>Заливку можно менять вместе с реальным остатком лимита — значок сам становится индикатором.</p>
<div class="row">{levels}</div>
</main></body></html>"""
open("mine.html", "w").write(page)
print("ok")
