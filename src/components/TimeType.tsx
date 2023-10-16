/**
 * Functions, components and type definintions for time types.
 * 
 * @author Jordon Davidson <jodavidson@mta.ca>
 * 
 * @since 0.1.0
 * @version 0.1.0
 */

import { invoke } from "@tauri-apps/api/tauri";
import React, { useState, useEffect } from "react"

/**
 * Represents a record from the timetypes table.
 * 
 * @since 0.1.0
 * @version 0.1.0
 */
type TimeType  = {
  id: Number,
  label: String,
  default_hours: Number,
}

/**
 * Get a timetype record by id.
 * 
 * @since 0.1.0
 * @version 0.1.0
 * 
 * @param id {Number} The id of the record to retrieve.
 * @returns A promise containing the desired timetype, or one with an error message in it.
 */
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

/**
 * Get all the timetypes from the database.
 * 
 * @since 0.1.0
 * @version 0.1.0
 * 
 * @returns A Promise containing an array of all tymetype, 
 *  or one tymetype with an error messsage in it.
 */
async function getAllTimeTypes() : Promise<Array<TimeType>> {
    const allTimeTypes = await invoke("list_timetypes")
        .catch( e => {
            const ttError : TimeType = {
                id: 0,
                label: `Error: ${e.message}`,
                default_hours: 0,
            }
            return [ttError]
        }) as Array<TimeType>

    return allTimeTypes
}

/**
 * TimeType component. 
 * 
 * @since 0.1.0
 * @version 0.1.0
 * 
 * @returns TimeType component
 */
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