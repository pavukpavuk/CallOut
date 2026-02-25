import "./footer.css";
import logo from "../assets/logo.png";

export function Footer(){
      return <>
            <div id="footer">
                <div className="footer-section-container">
                    <div className="footer-section" id="footer_section_l">
                        <h1>Never venture, never win!</h1>
                    </div>

                    {/* <div className="footer-section">
 
                    </div> */}


                    <img src={logo} id="logo"></img>
                    
                    <div className="footer-section">
                    </div>

                    <div className="footer-section" id="footer_section_r">

                        <div className="contact-info">
                            <a>Contact</a>
                            <a>FAQ</a>
                            <a>Donate</a>
                        </div>

                    </div>
                    
                    
                </div>
            </div> 
      </>
}