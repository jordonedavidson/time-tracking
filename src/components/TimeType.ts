import { invoke } from "@tauri-apps/api/tauri";

type TimeType  = {
  id: Number,
  label: String,
  default_hours: Number,
}

async function getTimeTypeById(id: Number) {
    const timeType = await invoke("get_timetype", {id: id})
        .catch (e => {
            return `<p className="error">${e.message}</p>`
        }) as TimeType
    
    return `<p><b>${timeType.label}</b>: ${timeType.default_hours}</p>`
}

export const TimeType = () => {
   const timeType = getTimeTypeById(1)

   return `<div>${timeType}</div>`
}