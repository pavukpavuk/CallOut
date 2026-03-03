import "./actions.css"

import menuArrow from "../assets/menu_arrow.png";
import { useState } from "react";

export function Actions(){

    const [open, setOpen] = useState(true);

    function toggleOpen(){
        setOpen(!open);
    };

    return <div className={`actions-main ${open ? "open" : ""}`}>
    
    <div className="actions-title">
        <h1>ACTIONS</h1> 
        <img className={`actions-menu-arrow ${open ? "open" : ""}`} onClick={toggleOpen}src={menuArrow}/>
    </div>
        <hr className={`action-button ${open ? "open" : ""}`}></hr>

    <a className={`callout-button action-button ${open ? "open" : ""}`}>CALLOUT</a>
    <a className={`action-button ${open ? "open" : ""}`}>VIEW ACTIVE CALLOUTS</a>
    <a className={`action-button ${open ? "open" : ""}`}>RESPOND TO CALLOUTS</a>
   
    
        
    
    

    </div>
}