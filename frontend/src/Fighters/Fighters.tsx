import "./fighters.css";

import black_belt_image from "./belt_vectors/black.svg";
import brown_belt_image from "./belt_vectors/brown.svg";
import purple_belt_image from "./belt_vectors/purple.svg";
import blue_belt_image from "./belt_vectors/blue.svg";
import white_belt_image from "./belt_vectors/white.svg";

import logo from "../assets/logo_no_arrow.png";

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
            fighter_rank = <img className="fighter-rank" src={black_belt_image}/>; 
            break;

        case "blue":
            fighter_rank = <img className="fighter-rank" src={blue_belt_image}/>;
            break; 

        case "purple": 
            fighter_rank = <img className="fighter-rank" src={purple_belt_image}/>;
            break; 

        case "brown":
            fighter_rank = <img className="fighter-rank" src={brown_belt_image}/>;
            break; 
        case "white":
            fighter_rank = <img className="fighter-rank" src={white_belt_image}/>;
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
            
            <button className="fighter-callout-button">
                <img className="callout-button-logo" src={logo}/>
            </button>
           
        </div>

        <div className="record-titles-container">
            <div>W:</div>
            <div>L:</div>
            <div>D:</div>
        </div>

        <div className="record-container">
            <div className="record-w">{fighter.wins}       </div> 
            <div className="record-l" >{fighter.losses}     </div>  
            <div className="record-d" >{fighter.draws}      </div> 
        </div>
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


    var searchForm = <>

        <div className="fighter-search-form">
            <div className="text-input-container"><span>Search Fighters:</span><input type="text" name="search-fighters"/></div>
            <br></br>
            <div className="fighter-search-button-container">
                <button className="fighter-search-button">Search</button>
            </div>
            
        </div>
    </>;




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
        {searchForm}
        {fightersHTML}
    </>
}