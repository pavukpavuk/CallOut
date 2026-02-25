import "./header.css";
import logo from "../assets/logo.png";

export function Header(){
      return <>
            <div id="header">
                  <img src={logo} id="logo"></img>
                 
                  <div id="navbar">
                        <div className="flex-link-container">
                              <a rel="icon" href="/fighters">
                                    FIGHTERS
                              </a>
                              <a rel="icon" href="/gym">
                                    GYMS
                              </a>
                              <a rel="icon" href="/fighters">
                                    SCOREBOARD
                              </a>
                              <a rel="icon" href="/matches">
                                    MATCHES
                              </a>
                        </div>
                  </div>
            </div> 
      </>
}


