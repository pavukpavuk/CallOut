
import React, {createContext ,useContext, useEffect, useState} from 'react';
import axios from 'axios';
import './userCRUD.css';

function UserInfo(username, email, password){
  this.username = username;
  this.email = email;
  this.password = password;
}

function UsernamePassword(username, password){
  this.username = username;
  this.password = password;
}

function UserUserFacing(username, email){
  this.username = username;
  this.email = email;
}


function UserPublic(username){
  this.username = username;
}

//todo:: 
// check for validity of email regex #STGKDQ
// check for strong password #YOAYJP
// error handling #CDPXTS
function CreateUser(){
  const [username, setUserName] = useState('');
  const [pword, setPword] = useState('');
  const [email, setEmail] = useState('');
  const userInfo = new UserInfo(username,email,pword);

  
  const handleInputName = (event) => {
    setUserName(event.target.value);
  };
  const handleInputPword = (event) => {
    setPword(event.target.value);
  };
  const handleInputEmail = (event) => {
    setEmail(event.target.value);
  };

  const handleSubmit = (event) =>{
    event.preventDefault();

    axios.post(
      `${process.env.REACT_APP_API_URL}/api/users`,

      userInfo,

      {
        withCredentials: false
      }
    ).then(res => {
      console.log(res.status);
    }).catch( err => {
      console.log(err.code);
    })
  };

  return (
    <div className={"input-module bordered"}>
      <h1>Create User</h1>


      <form onSubmit={handleSubmit}>
        <div className='label-and-input-container'>
          <label for= "name" >Username: </label>
          <input
            className='input-box'
            id="name"
            name="name"
            type="text"
            value={username}
            
            onChange={handleInputName}  
            />
        </div>
       
        <div className='label-and-input-container'>
          <label for= "email">Email: </label>
          <input
            className='input-box'
            id="email"
            name="email"
            type="text"
            value={email}
            
            onChange={handleInputEmail}  
          />
        </div>
        

        <div className='label-and-input-container'>
          <label for= "password">Password: </label>
          <input
            className='input-box'
            id="password"
            name="password"
            type="password"
            value={pword}
            onChange={handleInputPword}  
          />
        </div>
        <br/>
        <button type="submit" className='centered-button'>Submit</button>

      </form>
    </div>
  );
}

function FindAllUsers(){
  const [foundUsers, setUsers] = useState([]);

  const [status, setStatus] = useState(null);
  const [errMessage, setErrMessage] = useState(null);


  const findAllUsers = (e) =>{
    e.preventDefault();
    setStatus("searching");
    setUsers([]);

    axios.get(
      `${process.env.REACT_APP_API_URL}/api/users`
    )
    .then((res) => { 
      //found all users
      setStatus("found");
      setUsers(res.data);
    })
    .catch( (e) => {  //#WERTY
      setStatus("error");

      //no response from server
      if(!e.response){
        setErrMessage("No response from server");
      }

      else{
        setErrMessage(e.response.data?.message || "Unknown server error");
      }
      
      setUsers([]);
    });
  }

  
  let userInfoHTML;

  if(status === "found"){
    userInfoHTML = foundUsers.map( (user) => {
      return <>
        <div >
          Username: {user.username} 
        </div>
      </>
    });
  }

  if(status === "searching"){
    userInfoHTML = <div>Searching...</div>;
  }

  if(status === "error"){
    userInfoHTML = <div>No users found: {errMessage} </div>;
  }
   
 return (
    <>
      <div className={"find-all-users bordered"}>
        <h1>Find All Users</h1>
        <button type="submit" className='centered-button' onClick={findAllUsers}>Find</button>
      </div>

      {userInfoHTML}
    </>
  );
}

function FindUser(){
  const [usernameToFind,setUsernameToFind] = useState("");
  const [foundUsers, setFoundUsers] = useState([]);

  const [status, setStatus] = useState(null);
  const [errMessage, setErrMessage] = useState(null);

  function handleInputUsernameToFind(event){
    setUsernameToFind(event.target.value);
  }

  async function handleSubmit(event){
    event.preventDefault();
    setStatus("searching");
    try{
      const res = await axios.get(
        `${process.env.REACT_APP_API_URL}/api/users/${usernameToFind}`
      )
      setFoundUsers(res.data);
      setStatus("success");
    }
    catch(err){
        setStatus("error");

        if(!err.response){
          setErrMessage("No response from server");
        }
        else{
          setErrMessage(err.response.data.message);
        }
    }
  }


  let foundusersHTML = "";

  switch(status){

    case "success":
        foundusersHTML = <> </>;
        foundusersHTML =  
        <>
          <h1>{foundUsers.length} { foundUsers.length > 1 ? "users" : "user"} found:</h1>
          {
          foundUsers.map((user) => {
            return <>
              <div>
               {user.username} 
              </div>
            </>
            })
          }
          </>;
    break;

    case "searching":
      foundusersHTML = 
        <>
          <p>Searching...</p>
        </>
    break; 

    case "error":
      
        foundusersHTML = 
          <>
            No user found: {errMessage}
          </>
    break;

    default:

    break;
  }


  let inputComponent =       
    <div className={"input-module bordered"}>
       
      <h1>Find User</h1>

      <form onSubmit={handleSubmit}>
        <div className='label-and-input-container'>
          <label for= "name">Username: </label>
          <input
            id="name"
            name="username"
            type="text"
            value={usernameToFind}
            onChange={handleInputUsernameToFind}  
            required
            />
        </div>
        <br/>

        <button type="submit" className='centered-button'>Submit</button>
      </form>

    </div>

 return (
    <>
      {inputComponent}
      {foundusersHTML}
    </>
  );
}


/* 
  TODO::  
    handle response errors.
    
*/

// const SessionContext = createContext("logged_out");

function Login(){
  const [username, setUserName] = useState('');
  const [pword, setPword] = useState('');
  const loginInfo = new UsernamePassword(username,pword);
  
  const [status, setStatus] = useState(null);
  const [failureMessage, setFailureMessage] = useState(null);

  
  const handleInputName = (event) => {
    setUserName(event.target.value);
  };
  const handleInputPword = (event) => {
    setPword(event.target.value);
  };

  const handleSubmit = (event) =>{
    event.preventDefault();
    setStatus("logging_in");

    

    axios.post(
      `${process.env.REACT_APP_API_URL}/api/users/session`,

      loginInfo,
      {
        withCredentials: true
      }

    ).then(res => {
      console.log(res.status);
      setStatus("success");
    }).catch( err => {
      setStatus("error");
      console.log(err.code);
      if(err.response){
        setFailureMessage(err.code);
      }
      else{
        // setFailureMessage(JSON.stringify(err, null, 2));
        // alert(JSON.stringify(err, null, 2))
      }
      // setFailureMessage(err.response.data.message);
    })
  };

  let inputComponent =  
  <div className={"input-module bordered"}>
      <h1>Login</h1>


      <form onSubmit={handleSubmit}>
        <div className='label-and-input-container'>
          <label for= "name" >Username: </label>
          <input
            className='input-box'
            id="name"
            name="name"
            type="text"
            value={username}
            
            onChange={handleInputName}  
            />
        </div>

        <div className='label-and-input-container'>
          <label for= "password">Password: </label>
          <input
            className='input-box'
            id="password"
            name="password"
            type="password"
            value={pword}
            onChange={handleInputPword}  
          />
        </div>
        <br/>
        <button type="submit" className='centered-button'>Submit</button>

      </form>
  </div>

  let resultComponent = <></>
  switch( status){
    case "logging_in": 
      resultComponent = <p>Logging in...</p>;
    break;
    case "error": 
      resultComponent = <p>Error: {failureMessage}</p>;
    break;
    case "success": 
      resultComponent = <p>Success</p>;
    break;
    
  }
  

  return (
    <>
      {inputComponent}
      {resultComponent}
    </>
  );
}

function Logout() {

  const [status, setStatus] = useState("idle");
  const handleLogout = () => {  
      setStatus("logging_out");
      axios.delete(`${process.env.REACT_APP_API_URL}/api/users/session`,
        {
          withCredentials: true
        }
      ).then(()=>{ 
        
        setStatus("successfully_logged_out");
        
      }).catch(()=>{
        setStatus("failed_to_log_out");

      })
    
  }
  let button = <button className='centered-button' onClick={handleLogout}>Logout</button>
  return <>
    {button}
    {status}
  </>
}
function LoggedInAs(){

  const [user, setUser] = useState(null);
  const [sessionRetrieved, setSessionRetrieved] = useState(null);
  //send request with cookie. 
 
  useEffect(() => {
    axios.get( `${process.env.REACT_APP_API_URL}/api/users/session`, {withCredentials: true})
      .then((res) => {
        setUser(res.data);
        setSessionRetrieved("success");
      })
      .catch( ()=>{
        setSessionRetrieved("error");
      })
  }, []);
    
 
  
  let userHTML; 

  if(sessionRetrieved == "success"){
    userHTML = userHTML =  <div> User: {user.username} <br></br>Email {user.email} <br></br>User Agent: {user.user_agent} </div>;
  }
  if(sessionRetrieved == "error"){
    userHTML = <div>You are not logged in!</div>;
  }


  return <>
    {/* <button type="button" className='centered-button' onClick={checkSession}> Check session</button> */}
    {/* {useContext(SessionContext)} */}
    {userHTML}
  </>
}

export function UserCRUD(){
    return (
        <>
            <CreateUser/>

            <FindAllUsers/>

            <FindUser/>

            {/* <SessionContext> */}
              <Login/>  
              <Logout/>
            {/* </SessionContext> */}

            <LoggedInAs/>
        </>
    );
}


