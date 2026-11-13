import abliteration from "./logos/abliteration.svg?raw";
import claude from "./logos/claude.svg?raw";
import codex from "./logos/codex.svg?raw";
import copilot from "./logos/copilot.svg?raw";
import cursor from "./logos/cursor.svg?raw";
import deepseek from "./logos/deepseek.svg?raw";
import gemini from "./logos/gemini.svg?raw";
import jetbrains from "./logos/jetbrains.svg?raw";
import kimi from "./logos/kimi.svg?raw";
import opencode from "./logos/opencode_go.svg?raw";
import openrouter from "./logos/openrouter.svg?raw";
import telegram from "./logos/telegram.svg?raw";
import windsurf from "./logos/windsurf.svg?raw";

/** Logo drawing and its colour per service. No colour = follows the text colour. */
const logos: Record<string, { svg: string; color?: string }> = {
  abliteration: { svg: abliteration },
  claude: { svg: claude, color: "#D97757" },
  codex: { svg: codex },
  cursor: { svg: cursor },
  copilot: { svg: copilot },
  openrouter: { svg: openrouter, color: "#6467F2" },
  deepseek: { svg: deepseek, color: "#4D6BFE" },
  gemini: { svg: gemini, color: "#4E8CF7" },
  jetbrains: { svg: jetbrains },
  kimi: { svg: kimi },
  opencode_go: { svg: opencode },
  windsurf: { svg: windsurf },
  telegram: { svg: telegram, color: "#26A5E4" },
};

/** A small square with the service logo; a letter when there is no logo yet. */
export function logo(id: string, name: string): HTMLElement {
  const box = document.createElement("span");
  box.className = "logo";
  const known = logos[id];
  if (known) {
    box.innerHTML = known.svg;
    box.querySelector("title")?.remove();
    const svg = box.querySelector("svg");
    svg?.setAttribute("fill", "currentColor");
    svg?.setAttribute("aria-hidden", "true");
    if (known.color) box.style.color = known.color;
    box.classList.add(`logo-${id}`);
  } else {
    box.classList.add("letter");
    box.textContent = name.slice(0, 1).toUpperCase();
  }
  return box;
}
