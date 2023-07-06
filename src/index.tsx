/**
 * Basic boilerplate to place the App component on the index.html page
 * at the div#app node.
 *
 * @author Jordon Davidson <jodavidson@mta.ca>
 *
 * @since 1.0.0
 * @version 1.0.0
 */

import React from "react";
import {Root, createRoot} from "react-dom/client"
import { App } from "./components/App";
 

const container: HTMLElement = document.getElementById("app") as HTMLElement;
const app_root: Root = createRoot(container);
app_root.render(<App />);