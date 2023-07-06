import { invoke } from "@tauri-apps/api/tauri";
import React, { useState, useEffect } from "react"

type TimeType  = {
  id: Number,
  label: String,
  default_hours: Number,
}

async function getTimeTypeById(id: Number) : Promise<TimeType> {
    console.log("in getTimeTypeById")
    const timeType = await invoke("get_timetype", {id: id})
        .catch (e => {
            const ttError : TimeType = {
                id: 0,
                label: `Error ${e.message}`,
                default_hours: 0,
            }
            return ttError
        }) as TimeType
    
    console.log(timeType)
    return timeType
}

export const TimeType : React.FC = () => {

    const [timeType, setTimeType] = useState<TimeType | null>(null)

    useEffect(() => {
        getTimeTypeById(1)
            .then(t => setTimeType(t))
            .catch(e => console.log(e))
    },[])

    if (timeType) {
        return (
            <p>
                <b>{timeType.id.toString()}</b>: {timeType.label}
            </p>
        )
    }

    return <p>Loading</p>
}