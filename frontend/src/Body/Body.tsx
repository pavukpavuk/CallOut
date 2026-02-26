import "./Body.css";
import {Fighters} from "../Fighters/Fighters"
import {Actions} from "../Actions/Actions"
import {Notifications} from "../Notifications/Notifications"

export function Body(){
    return <>
        <div id="body">

            

            <div id="page-content">
                <div id="left-side-content">
                    <Actions/>
                </div>
                <div id="safe-zone">
                    <Fighters/>
                </div>
                <div id="right-side-content">
                    <Notifications/>
                </div>
                
            </div>
        </div> 
    </>
}


