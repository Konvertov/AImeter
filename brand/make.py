# Draws the logo ideas as pixel art: every character of a map is one square.
#   '#' orange   'o' ink (white on dark, near-black on light)   '+' dim   '.' empty
ORANGE = "#D97757"

IDEAS = [
    ("1", "Робот", "Robot", "Голова робота, у которого рот — полоска лимита. Персонаж сразу объясняет, что делает приложение.", [
        ".....oo.....",
        ".....oo.....",
        "..########..",
        ".##########.",
        ".##oo##oo##.",
        ".##oo##oo##.",
        ".##########.",
        ".#wwwwww++#.",
        ".##########.",
        "..########..",
    ]),
    ("2", "Батарейка", "Battery buddy", "Батарейка с глазами: заряд — это остаток лимита. Простая и добрая.", [
        "....oooo....",
        "..oooooooo..",
        "..o......o..",
        "..o.o..o.o..",
        "..o......o..",
        "..o######o..",
        "..o######o..",
        "..o######o..",
        "..oooooooo..",
    ]),
    ("3", "Сердце", "Life", "Как жизни в старых играх: сердце заполнено наполовину — столько лимита осталось.", [
        ".++...++.",
        "++++.++++",
        "+++++++++",
        "#########",
        ".#######.",
        "..#####..",
        "...###...",
        "....#....",
    ]),
    ("4", "Колба", "Potion", "Игровая колба с зельем: запас сил, который тратится и восполняется.", [
        "...oooo...",
        "....oo....",
        "....oo....",
        "...o..o...",
        "..o....o..",
        ".o.####.o.",
        ".o######o.",
        ".o######o.",
        "..oooooo..",
    ]),
    ("5", "Огонёк", "Spark", "Живой огонёк с глазами: энергия, которая горит, пока есть лимит.", [
        ".....#....",
        "....##....",
        "...###.#..",
        "..#######.",
        ".########.",
        ".##o##o##.",
        ".########.",
        ".###ww###.",
        "..######..",
    ]),
    ("6", "Сова", "Owl", "Сова-наблюдатель: следит за лимитами, пока вы работаете.", [
        ".#......#.",
        ".##....##.",
        ".########.",
        ".#oo##oo#.",
        ".#ok##ok#.",
        ".###ww###.",
        ".########.",
        ".########.",
        "..#.##.#..",
    ]),
    ("7", "Буква M", "Letter M", "Пиксельная «M» от слова Meter. Строгий знак без персонажа.", [
        "##......##",
        "###....###",
        "####..####",
        "##.####.##",
        "##..##..##",
        "##......##",
        "##......##",
    ]),
    ("8", "Полоска-друг", "Bar buddy", "Сама полоска лимита, но с глазами: заполненная часть — живая.", [
        ".oooooooooooo.",
        "o#######.....o",
        "o#o##o##.....o",
        "o#######.....o",
        ".oooooooooooo.",
    ]),
]


def squares(rows, ink, dim, size, pad):
    """SVG <rect>s for a map, centred in a size x size tile."""
    width = max(len(r) for r in rows)
    height = len(rows)
    cell = (size - 2 * pad) / max(width, height)
    left = (size - width * cell) / 2
    top = (size - height * cell) / 2
    out = []
    for y, row in enumerate(rows):
        for x, ch in enumerate(row):
            if ch == ".":
                continue
            fill = {"#": ORANGE, "o": ink, "+": dim, "w": "#ffffff", "k": "#14161c"}[ch]
            # a hair of overlap hides seams between neighbouring squares
            out.append(
                f'<rect x="{left + x * cell:.2f}" y="{top + y * cell:.2f}" '
                f'width="{cell + 0.4:.2f}" height="{cell + 0.4:.2f}" fill="{fill}"/>'
            )
    return "".join(out)


def tile(rows, dark, size=160):
    bg, ink, dim = ("#14161c", "#f2f4f8", "#3a4050") if dark else ("#ffffff", "#14161c", "#d5d9e0")
    radius = size * 0.225
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" width="{size}" height="{size}">'
        f'<rect width="{size}" height="{size}" rx="{radius:.1f}" fill="{bg}"/>'
        f'{squares(rows, ink, dim, size, size * 0.2)}</svg>'
    )


def mono(rows, color, size=18):
    """One-colour version, as it would sit in the menu bar."""
    width = max(len(r) for r in rows)
    height = len(rows)
    cell = size / max(width, height)
    left = (size - width * cell) / 2
    top = (size - height * cell) / 2
    out = []
    for y, row in enumerate(rows):
        for x, ch in enumerate(row):
            if ch in "#ow":
                out.append(
                    f'<rect x="{left + x * cell:.2f}" y="{top + y * cell:.2f}" '
                    f'width="{cell + 0.2:.2f}" height="{cell + 0.2:.2f}" fill="{color}"/>'
                )
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {size} {size}" width="{size}" height="{size}">'
        + "".join(out) + "</svg>"
    )


cards = []
for number, name, english, about, rows in IDEAS:
    cards.append(f"""
  <article>
    <header><b>{number}. {name}</b><i>{english}</i></header>
    <div class="tiles">{tile(rows, True)}{tile(rows, False)}</div>
    <div class="bars">
      <span class="bar dark">{mono(rows, "#f2f4f8")}<em>42%</em></span>
      <span class="bar light">{mono(rows, "#14161c")}<em>42%</em></span>
      <span class="small">{tile(rows, True, 32)}{tile(rows, True, 64)}</span>
    </div>
    <p>{about}</p>
  </article>""")

page = f"""<!doctype html>
<html lang="ru"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>AI Meter — варианты логотипа</title>
<style>
  body {{ margin: 0; background: #eef0f3; color: #14161c; font: 15px/1.5 -apple-system, "Segoe UI", system-ui, sans-serif; }}
  main {{ max-width: 1180px; margin: 0 auto; padding: 36px 20px 60px; }}
  h1 {{ margin: 0 0 6px; font-size: 30px; letter-spacing: -0.02em; }}
  h1 span {{ color: {ORANGE}; }}
  .lead {{ margin: 0 0 28px; color: #5d6675; max-width: 60em; }}
  .grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(360px, 1fr)); gap: 18px; }}
  article {{ background: #fff; border: 1px solid #dfe3e8; border-radius: 16px; padding: 18px; }}
  header {{ display: flex; align-items: baseline; gap: 10px; margin-bottom: 14px; }}
  header b {{ font-size: 18px; }}
  header i {{ font-style: normal; color: #8a93a2; font-size: 13px; }}
  .tiles {{ display: flex; gap: 14px; }}
  .tiles svg {{ border-radius: 36px; box-shadow: 0 6px 18px rgba(20, 22, 28, 0.14); }}
  .bars {{ display: flex; align-items: center; gap: 10px; margin: 16px 0 12px; flex-wrap: wrap; }}
  .bar {{ display: inline-flex; align-items: center; gap: 6px; padding: 4px 10px; border-radius: 7px; font-size: 13px; }}
  .bar em {{ font-style: normal; font-weight: 600; font-variant-numeric: tabular-nums; }}
  .bar.dark {{ background: #23262e; color: #f2f4f8; }}
  .bar.light {{ background: #e6e8ec; color: #14161c; }}
  .small {{ display: inline-flex; align-items: flex-end; gap: 8px; margin-left: auto; }}
  article p {{ margin: 0; color: #5d6675; }}
  .note {{ margin-top: 26px; color: #5d6675; max-width: 62em; }}
</style></head><body><main>
<h1><span>AI</span> Meter — варианты логотипа</h1>
<p class="lead">Вторая подборка: восемь набросков в пиксельном стиле, теперь с персонажами и игровыми образами. У каждого: значок приложения на тёмном и на светлом фоне, одноцветная версия в том виде, как она будет стоять в строке меню рядом с числом, и мелкие размеры 32 и 64 точки.</p>
<div class="grid">{''.join(cards)}
</div>
<p class="note">Все варианты нарисованы с нуля на сетке из квадратов; цвет — тот же оранжевый, что у «AI» в названии. Это наброски формы: выбранный вариант можно доработать — поменять пропорции, толщину, добавить или убрать детали.</p>
</main></body></html>"""

open("logo-ideas.html", "w").write(page)
print("ok", len(IDEAS))
