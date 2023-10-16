/**
 * Main app layout and logic.
 * 
 * @author Jordon Davidson <jodavidson@mta.ca>
 * 
 * @since 0.1.0
 * @version 0.1.0
 */

import React from "react";
import { TimeType } from "./TimeType";
import { UserType } from "./User";

export const App: React.FC = () => {

    return (
        <>
        <div>
            <TimeType />
        </div>
        <div>
            <UserType />
        </div>
        </>
    )
}