# Six more logo ideas, each a different thought, drawn in vectors on a 1024 box.
import itertools
ORANGE = "#D97757"
_ids = itertools.count()

def smile(ink, fill, dim, level=0.62):
    """A face whose smile is the gauge: the orange part of the smile is what is left."""
    arc = "M292 580 Q512 820 732 580"
    return (
        f'<circle cx="388" cy="408" r="50" fill="{ink}"/><circle cx="636" cy="408" r="50" fill="{ink}"/>'
        f'<path d="{arc}" fill="none" stroke="{dim}" stroke-width="76" stroke-linecap="round"/>'
        f'<path d="{arc}" fill="none" stroke="{fill}" stroke-width="76" stroke-linecap="round" '
        f'pathLength="100" stroke-dasharray="{level * 100:.0f} 100"/>'
    )

def bubble(ink, fill, dim, level=0.62):
    """A chat bubble with a limit bar inside: a conversation with an AI, and how much of it is left."""
    body = ("M312 222 H712 a130 130 0 0 1 130 130 V562 a130 130 0 0 1 -130 130 H506 "
            "L352 832 V692 H312 a130 130 0 0 1 -130 -130 V352 a130 130 0 0 1 130 -130 Z")
    return (
        f'<path d="{body}" fill="none" stroke="{ink}" stroke-width="58" stroke-linejoin="round"/>'
        f'<rect x="292" y="420" width="440" height="76" rx="38" fill="{dim}"/>'
        f'<rect x="292" y="420" width="{440 * level:.0f}" height="76" rx="38" fill="{fill}"/>'
    )

def arcs(ink, fill, dim, level=0.5):
    """Two dials one inside the other: the week outside, the session inside."""
    def dial(radius):
        dx, dy = 0.866 * radius, 0.5 * radius
        return f"M{512 - dx:.1f} {570 + dy:.1f} A{radius} {radius} 0 1 1 {512 + dx:.1f} {570 + dy:.1f}"
    out = ""
    for radius, color, part in ((312, ink, 0.72), (176, fill, level)):
        out += (f'<path d="{dial(radius)}" fill="none" stroke="{dim}" stroke-width="84" stroke-linecap="round"/>'
                f'<path d="{dial(radius)}" fill="none" stroke="{color}" stroke-width="84" stroke-linecap="round" '
                f'pathLength="100" stroke-dasharray="{part * 100:.0f} 100"/>')
    return out

def letter_m(ink, fill, dim, level=0.5):
    """The letter M of Meter in two halves: what is spent and what is left."""
    return (
        f'<path d="M244 742 V300 L512 640" fill="none" stroke="{ink}" stroke-width="96" stroke-linecap="round" stroke-linejoin="round"/>'
        f'<path d="M512 640 L780 300 V742" fill="none" stroke="{fill}" stroke-width="96" stroke-linecap="round" stroke-linejoin="round"/>'
    )

def pie(ink, fill, dim, level=0.66):
    """A disc that is eaten away like a clock face: the orange slice is the limit still left."""
    import math
    angle = 2 * math.pi * level
    x, y = 512 + 310 * math.sin(angle), 512 - 310 * math.cos(angle)
    big = 1 if level > 0.5 else 0
    return (
        f'<circle cx="512" cy="512" r="310" fill="{dim}"/>'
        f'<path d="M512 512 L512 202 A310 310 0 {big} 1 {x:.1f} {y:.1f} Z" fill="{fill}"/>'
        f'<circle cx="512" cy="512" r="58" fill="{ink}"/>'
    )

def drop(ink, fill, dim, level=0.55):
    """A drop that is partly full: tokens as a liquid that runs out and is topped up again."""
    shape = "M512 172 C512 172 252 470 252 632 a260 260 0 0 0 520 0 C772 470 512 172 512 172 Z"
    uid = f"drop{next(_ids)}"
    top = 892 - 600 * level
    return (
        f'<clipPath id="{uid}"><path d="{shape}" transform="translate(512 600) scale(0.74) translate(-512 -600)"/></clipPath>'
        f'<rect x="200" y="{top:.0f}" width="640" height="760" fill="{fill}" clip-path="url(#{uid})"/>'
        f'<path d="{shape}" fill="none" stroke="{ink}" stroke-width="58" stroke-linejoin="round"/>'
    )

IDEAS = [
    ("1. Улыбка", smile, "Лицо, у которого улыбка — это шкала. Пока лимит есть, улыбка оранжевая. Тёплый и запоминающийся знак, не похожий ни на чей."),
    ("2. Диалог", bubble, "Облачко чата с полоской лимита внутри. Прямо говорит: это про разговоры с нейросетью и про то, сколько их осталось."),
    ("3. Две шкалы", arcs, "Две дуги одна в другой: снаружи неделя, внутри сессия. Ровно те два лимита, которые приложение показывает первыми."),
    ("4. Буква M", letter_m, "Буква M из названия Meter, разделённая на потраченное и оставшееся. Самый строгий и «фирменный» вариант."),
    ("5. Остаток", pie, "Диск, который убывает, как циферблат. Оранжевый сектор — сколько лимита ещё есть."),
    ("6. Капля", drop, "Капля, заполненная до уровня: токены как жидкость, которая расходуется и доливается при сбросе."),
]

def icon(draw, dark=True, size=1024, **kw):
    bg, ink, dim = ("#14161c", "#f2f4f8", "#333947") if dark else ("#ffffff", "#14161c", "#dde0e6")
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="{size}" height="{size}">'
            f'<rect width="1024" height="1024" rx="230" fill="{bg}"/>{draw(ink, ORANGE, dim, **kw)}</svg>')

def bare(draw, color, size=20):
    return (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="120 120 784 784" width="{size}" height="{size}">'
            f'{draw(color, color, "none")}</svg>')

cards = ""
for index, (name, draw, about) in enumerate(IDEAS):
    levels = "" if draw is letter_m else "".join(icon(draw, True, 64, level=l) for l in (0.2, 0.45, 0.7, 0.95))
    note = "" if draw is letter_m else "<i>уровень может меняться вместе с остатком лимита</i>"
    open(f"taste-{index + 1}.svg", "w").write(icon(draw, True))
    cards += f"""
<article>
  <h2>{name}</h2>
  <div class="row">{icon(draw, True, 190)}{icon(draw, False, 190)}{icon(draw, True, 64)}{icon(draw, True, 32)}{icon(draw, True, 16)}</div>
  <div class="row"><div class="lock">{icon(draw, True, 56)}<span><b>AI</b> Meter</span></div>
    <span class="bar dark">{bare(draw, "#f2f4f8")}42%</span><span class="bar light">{bare(draw, "#14161c")}42%</span></div>
  <div class="row levels">{levels}{note}</div>
  <p>{about}</p>
</article>"""

page = f"""<!doctype html><html lang="ru"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>AI Meter — ещё логотипы</title><style>
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
<h1><span>AI</span> Meter — ещё логотипы на мой вкус</h1>
<p class="lead">Шесть разных мыслей. У каждого: значок на тёмном и светлом фоне, мелкие размеры, вид с названием и в строке меню, и уровни заполнения.</p>
<div class="grid">{cards}</div>
</main></body></html>"""
open("taste.html", "w").write(page)
print("ok")
