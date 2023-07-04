import { invoke } from "@tauri-apps/api/tauri";

type TimeType  = {
  id: Number,
  label: String,
  default_hours: Number,
}

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let listingEl: HTMLDivElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

async function timeTypeListing() {
  if (listingEl) {
    const timeTypes = await invoke("list_timetypes")
      .catch(e => {
        const p = document.createElement('p')
        p.innerText = e.message
        p.className = 'error'
        listingEl?.appendChild(p)
      }) as Array<TimeType>

    timeTypes.forEach(t => {
      const p = document.createElement('p')
      p.innerHTML = `<b>${t.id}</b>: ${t.label} - ${t.default_hours}`
      listingEl?.appendChild(p)
    })
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  listingEl = document.querySelector("#timetypes");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
  timeTypeListing();
});
