# Battery-themed logo ideas, drawn in vectors on a 1024 box.
ORANGE = "#D97757"

def horizontal(ink, fill, dim, level=0.62):
    """A battery lying on its side; the orange bar is what is left."""
    width = 484 * level
    return (
        f'<rect x="182" y="352" width="600" height="320" rx="92" fill="none" stroke="{ink}" stroke-width="56"/>'
        f'<rect x="816" y="446" width="56" height="132" rx="26" fill="{ink}"/>'
        f'<rect x="240" y="410" width="{width:.0f}" height="204" rx="42" fill="{fill}"/>'
    )

def segments(ink, fill, dim, level=0.75):
    """The same battery with the charge cut into four cells."""
    lit = round(level * 4)
    cells = "".join(
        f'<rect x="{244 + i * 122}" y="414" width="96" height="196" rx="30" fill="{fill if i < lit else dim}"/>'
        for i in range(4)
    )
    return (
        f'<rect x="182" y="352" width="600" height="320" rx="92" fill="none" stroke="{ink}" stroke-width="56"/>'
        f'<rect x="816" y="446" width="56" height="132" rx="26" fill="{ink}"/>' + cells
    )

def spark(ink, fill, dim, level=0.62, hole="#14161c"):
    """A standing battery with a four-point spark, the usual sign for AI, in the charge."""
    top = 290 + 484 * (1 - level)
    star = "M512 548 C524 604 540 620 596 632 C540 644 524 660 512 716 C500 660 484 644 428 632 C484 620 500 604 512 548 Z"
    return (
        f'<rect x="442" y="150" width="140" height="60" rx="26" fill="{ink}"/>'
        f'<rect x="322" y="232" width="380" height="600" rx="96" fill="none" stroke="{ink}" stroke-width="56"/>'
        f'<path d="M380 {top:.0f} H644 V734 a40 40 0 0 1 -40 40 H420 a40 40 0 0 1 -40 -40 Z" fill="{fill}"/>'
        f'<path d="{star}" fill="{hole}"/>'
    )

def liquid(ink, fill, dim, level=0.58):
    """A plump battery with a wavy surface, as if the charge were a drink."""
    y = 322 + 440 * (1 - level)
    wave = (f"M272 {y:.0f} C332 {y-44:.0f} 392 {y-44:.0f} 452 {y:.0f} S572 {y+44:.0f} 632 {y:.0f} "
            f"S722 {y-40:.0f} 752 {y-14:.0f} V722 a44 44 0 0 1 -44 44 H316 a44 44 0 0 1 -44 -44 Z")
    return (
        f'<rect x="432" y="176" width="160" height="60" rx="26" fill="{ink}"/>'
        f'<rect x="212" y="262" width="600" height="564" rx="150" fill="none" stroke="{ink}" stroke-width="58"/>'
        f'<path d="{wave}" fill="{fill}"/>'
    )

IDEAS = [
    ("1. Заряд", horizontal, "Классическая батарейка на боку. Оранжевая часть — сколько лимита осталось. Самый понятный вариант: его не нужно объяснять."),
    ("2. Ячейки", segments, "Та же батарейка, но заряд разбит на четыре ячейки, как индикатор на телефоне. Потраченные ячейки гаснут."),
    ("3. Искра", spark, "Стоячая батарейка, а в заряде вырезана четырёхлучевая искра — привычный знак нейросетей. Читается как «заряд ИИ»."),
    ("4. Напиток", liquid, "Пухлая батарейка с волнистой поверхностью, будто заряд — жидкость. Самая мягкая и дружелюбная."),
]

def icon(draw, dark=True, size=1024, **kw):
    bg, ink, dim = ("#14161c", "#f2f4f8", "#333947") if dark else ("#ffffff", "#14161c", "#d9dde3")
    if draw is spark:
        kw["hole"] = "#f6e7e1" if not dark else "#14161c"
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="{size}" height="{size}">'
            f'<rect width="1024" height="1024" rx="230" fill="{bg}"/>{draw(ink, ORANGE, dim, **kw)}</svg>')

def bare(draw, color, size=20):
    kw = {"hole": "none"} if draw is spark else {}
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="110 110 804 804" width="{size}" height="{size}">'
            f'{draw(color, color, "none", **kw)}</svg>')

cards = ""
for index, (name, draw, about) in enumerate(IDEAS):
    levels = "".join(icon(draw, True, 64, level=l) for l in (0.25, 0.5, 0.75, 1.0))
    open(f"battery-{index + 1}.svg", "w").write(icon(draw, True))
    cards += f"""
<article>
  <h2>{name}</h2>
  <div class="row">{icon(draw, True, 190)}{icon(draw, False, 190)}{icon(draw, True, 64)}{icon(draw, True, 32)}{icon(draw, True, 16)}</div>
  <div class="row"><div class="lock">{icon(draw, True, 56)}<span><b>AI</b> Meter</span></div>
    <span class="bar dark">{bare(draw, "#f2f4f8")}42%</span><span class="bar light">{bare(draw, "#14161c")}42%</span></div>
  <div class="row levels">{levels}<i>уровень может меняться вместе с остатком лимита</i></div>
  <p>{about}</p>
</article>"""

page = f"""<!doctype html><html lang="ru"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>AI Meter — логотип с батарейкой</title><style>
body{{margin:0;background:#eef0f3;color:#14161c;font:15px/1.5 -apple-system,"Segoe UI",system-ui,sans-serif}}
main{{max-width:1180px;margin:0 auto;padding:36px 20px 60px}}
h1{{margin:0 0 6px;font-size:30px;letter-spacing:-.02em}} h1 span{{color:{ORANGE}}}
.lead{{margin:0 0 24px;color:#586170}}
.grid{{display:grid;grid-template-columns:repeat(auto-fill,minmax(520px,1fr));gap:18px}}
article{{background:#fff;border:1px solid #dfe3e8;border-radius:18px;padding:20px}}
h2{{margin:0 0 14px;font-size:19px}}
p{{margin:14px 0 0;color:#586170}}
.row{{display:flex;flex-wrap:wrap;gap:14px;align-items:flex-end;margin-top:14px}}
.row>svg{{border-radius:22.5%;box-shadow:0 8px 22px rgba(20,22,28,.15)}}
.bar{{display:inline-flex;align-items:center;gap:7px;padding:5px 12px;border-radius:8px;font-size:13px;font-weight:600;align-self:center}}
.dark{{background:#23262e;color:#f2f4f8}} .light{{background:#dfe2e7;color:#14161c}}
.lock{{display:flex;align-items:center;gap:12px;background:#14161c;color:#f2f4f8;border-radius:14px;padding:12px 20px;font-size:30px;font-weight:700;letter-spacing:-.02em}}
.lock b{{color:{ORANGE}}}
.levels i{{font-style:normal;font-size:13px;color:#586170;align-self:center}}
</style></head><body><main>
<h1><span>AI</span> Meter — логотип с батарейкой</h1>
<p class="lead">Четыре варианта на одну тему. У каждого: значок на тёмном и светлом фоне, мелкие размеры, вид с названием и в строке меню, и четыре уровня заряда.</p>
<div class="grid">{cards}</div>
</main></body></html>"""
open("battery.html", "w").write(page)
print("ok")
