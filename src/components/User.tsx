/**
 * Functions, component and type definintions for user types.
 * 
 * @author Jordon Davidson <jodavidson@mta.ca>
 * 
 * @since 0.1.0
 * @version 0.1.0
 */

import { invoke } from "@tauri-apps/api/tauri"
import React, {useState, useEffect } from "react"

/**
 * Represents a record from the users table.
 * 
 * @since 0.1.0
 * @verson 0.1.0
 */
type UserType = {
    id: Number,
    username: String,
    display_name: String,
    roles: Array<String>,
    totals: Object,
}

/**
 * Get a user record by username.
 * 
 * @since 0.1.0
 * @version 0.1.0
 * 
 * @param username {String} The username of the user record to retrieve.
 * @returns A promise containing the usertype, or one with an error message in it.
 */
async function getUserByUsername(username: String) : Promise<UserType> {
    const userType = await invoke("get_user_by_username", {username: username})
        .catch(e => {
            const utError : UserType = {
                id: 0,
                username: "error",
                display_name: `Error ${e.message}`,
                roles: ['error'],
                totals: {},
            }
            return utError
        }) as UserType

    console.log(userType)

    return userType
}

/**
 * UserType component
 * 
 * @since 0.1.0
 * @version 0.1.0
 * 
 * @returns UserType component
 */
export const UserType : React.FC = () => {
    const [userType, setUserType] = useState<UserType | null>(null)

    useEffect(() => {
        getUserByUsername('jodavidson')
            .then(u => setUserType(u))
            .catch(e => console.log(e))
    },[])

    if (userType) {
        return (
            <p>
                <b>{userType.id.toString()}</b>: {userType.username} - {userType.display_name}
            </p>
        )
    }

    return <p>Loading User</p>
}