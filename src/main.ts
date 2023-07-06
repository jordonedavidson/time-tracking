import { invoke } from "@tauri-apps/api/tauri";

type TimeType  = {
  id: Number,
  label: String,
  default_hours: Number,
}

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let listingEl: HTMLDivElement | null;
let ttEl: HTMLDivElement | null;

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
    console.log("listingEl is not null");
    
    await invoke("list_timetypes")
      .then(timeTypes => {
        timeTypes.forEach(t => {
          const p = document.createElement('p')
          p.innerHTML += `<b>${t.id}</b>: ${t.label} - ${t.default_hours}`
          listingEl?.appendChild(p)
        })
      })
      .catch(e => {
        const p = document.createElement('p')
        p.innerText = `List time types error: ${e.message}`
        p.className = 'error'
        listingEl?.appendChild(p)
      })
  }
}

async function getTimeTypeById(id: Number) : Promise<void> {
  if (ttEl) {
    const p = document.createElement('p')
    await invoke("get_timetype", {id: id})
        .then((t) => {
          p.innerHTML = `<b>${t.label}</b>: ${t.default_hours}`
        })
        .catch (e => {
          p.innerText = e.message
          p.className = 'error'
        })
    
    ttEl?.appendChild(p)
  }

}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  listingEl = document.querySelector("#timetypes");
  ttEl = document.querySelector("#timetype");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
  timeTypeListing();
  getTimeTypeById(1);
});
