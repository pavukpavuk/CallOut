import "./fighters.css";

import black_belt_image from "./belt_vectors/black.svg";
import brown_belt_image from "./belt_vectors/brown.svg";
import purple_belt_image from "./belt_vectors/purple.svg";
import blue_belt_image from "./belt_vectors/blue.svg";
import white_belt_image from "./belt_vectors/white.svg";

import logo from "../assets/logo.png";

import {useEffect, useState} from "react";
import axios from "axios";


interface Fighter { 
    name : string;
    picture_url: string;
    description: string;
    rank: string;
    wins: number;
    losses: number;
    draws: number;
    created_at: string;
    gym_id_fk: number;
}

interface FighterProps{
    fighter: Fighter
}


function Fighter({fighter}: FighterProps){
    var fighter_rank = <></>;
    
    switch(fighter.rank){
        case "black":
            fighter_rank = <img src={black_belt_image}/>; 
            break;

        case "blue":
            fighter_rank = <img src={blue_belt_image}/>;
            break; 

        case "purple": 
            fighter_rank = <img src={purple_belt_image}/>;
            break; 

        case "brown":
            fighter_rank = <img src={brown_belt_image}/>;
            break; 
        case "white":
            fighter_rank = <img src={white_belt_image}/>;
            break; 
    }

    return <div className={"fighter-container"}>

        <div className="fighter-headshot-container">
            <img src={`/assets/fighter_headshots/${fighter.picture_url}`}/>
        </div>

        {fighter_rank}

        
        <div className="fighter-text-container">
            <h1> {fighter.name}       </h1> 
            <p>  {fighter.description}</p> 
            
                <button className="fighter-callout-button"><img className="callout-button-logo" src={logo}/></button>
           
        </div>
        <div>{fighter.wins}       </div> 
        <div>{fighter.losses}     </div>  
        <div>{fighter.draws}      </div> 
    </div>
}


export function Fighters(){
  
    // if(import.meta.env.VITE_DEV === "true"){
    //     console.log("DEV!");
    // } else{
    //     console.log("NOT DEV!");
    // }

    const [fighterArr, setFighterArr]=useState<Fighter[]>([]);
    const [fighterArrState, setFighterArrState] = useState("");
    
    useEffect(() => {
        axios.get('/assets/fighters.json', //just for dev
        {
            headers : { 
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            }
        })
        .then(res => {
            setFighterArr(res.data);
            setFighterArrState("received");
        });
        
    }, []);


    var fightersHTML;
    if(fighterArrState === "received"){
            fightersHTML = fighterArr.map( (fighter, idx) =>{
                return <Fighter key={idx} fighter={fighter}/>;
            });
        
    }
    else{
        return "Noting";
    }


    return <>
        {fightersHTML}
    </>
}