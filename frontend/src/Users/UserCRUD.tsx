import React, { useEffect, useState } from 'react';
import axios from 'axios';
import type {AxiosResponse} from 'axios';
import './userCRUD.css';

// ----- Type Definitions -----
interface UserInfo {
  username: string;
  email: string;
  password: string;
}

interface UsernamePassword {
  username: string;
  password: string;
}

interface UserUserFacing {
  username: string;
  email: string;
}

interface UserPublic {
  username: string;
}

interface UserSession {
  username: string;
  email: string;
  user_agent: string;
}

// ----- Components -----
export function CreateUser() {
  const [username, setUsername] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [email, setEmail] = useState<string>('');

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const userInfo: UserInfo = { username, email, password };

    axios
      .post(`${import.meta.env.VITE_API_URL}/api/users`, userInfo, { withCredentials: false })
      .then(res => {
        console.log(res.status);
      })
      .catch(err => {
        console.log(err.code);
      });
  };

  return (
    <div className="input-module bordered">
      <h1>Create User</h1>
      <form onSubmit={handleSubmit}>
        <div className="label-and-input-container">
          <label htmlFor="name">Username: </label>
          <input
            className="input-box"
            id="name"
            name="name"
            type="text"
            value={username}
            onChange={e => setUsername(e.target.value)}
          />
        </div>

        <div className="label-and-input-container">
          <label htmlFor="email">Email: </label>
          <input
            className="input-box"
            id="email"
            name="email"
            type="text"
            value={email}
            onChange={e => setEmail(e.target.value)}
          />
        </div>

        <div className="label-and-input-container">
          <label htmlFor="password">Password: </label>
          <input
            className="input-box"
            id="password"
            name="password"
            type="password"
            value={password}
            onChange={e => setPassword(e.target.value)}
          />
        </div>

        <br />
        <button type="submit" className="centered-button">
          Submit
        </button>
      </form>
    </div>
  );
}

// ----- Find All Users -----
export function FindAllUsers() {
  const [foundUsers, setUsers] = useState<UserUserFacing[]>([]);
  const [status, setStatus] = useState<'idle' | 'searching' | 'found' | 'error'>('idle');
  const [errMessage, setErrMessage] = useState<string | null>(null);

  const findAllUsers = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    setStatus('searching');
    setUsers([]);

    axios
      .get<UserUserFacing[]>(`${import.meta.env.VITE_API_URL}/api/users`)
      .then(res => {
        setUsers(res.data);
        setStatus('found');
      })
      .catch(err => {
        setStatus('error');
        if (!err.response) {
          setErrMessage('No response from server');
        } else {
          setErrMessage(err.response.data?.message || 'Unknown server error');
        }
      });
  };

  let userInfoHTML;
  if (status === 'found') {
    console.log(foundUsers);
    userInfoHTML = foundUsers.map(user => <div key={user.username}>Username: {user.username}</div>);
  } else if (status === 'searching') {
    userInfoHTML = <div>Searching...</div>;
  } else if (status === 'error') {
    userInfoHTML = <div>No users found: {errMessage}</div>;
  }

  return (
    <>
      <div className="find-all-users bordered">
        <h1>Find All Users</h1>
        <button type="button" className="centered-button" onClick={findAllUsers}>
          Find
        </button>
      </div>
      {userInfoHTML}
    </>
  );
}

// ----- Find User by Username -----
export function FindUser() {
  const [usernameToFind, setUsernameToFind] = useState<string>('');
  const [foundUsers, setFoundUsers] = useState<UserUserFacing[]>([]);
  const [status, setStatus] = useState<'idle' | 'searching' | 'success' | 'error'>('idle');
  const [errMessage, setErrMessage] = useState<string | null>(null);

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setStatus('searching');
    try {
      const res: AxiosResponse<UserUserFacing[]> = await axios.get(
        `${import.meta.env.VITE_API_URL}/api/users/${usernameToFind}`
      );
      setFoundUsers(res.data);
      setStatus('success');
    } catch (err: any) {
      setStatus('error');
      if (!err.response) {
        setErrMessage('No response from server');
      } else {
        setErrMessage(err.response.data?.message);
      }
    }
  };

  let foundusersHTML;
  switch (status) {
    case 'success':
      foundusersHTML = (
        <>
          <h1>
            {foundUsers.length} {foundUsers.length > 1 ? 'users' : 'user'} found:
          </h1>
          {foundUsers.map(user => (
            <div key={user.username}>{user.username}</div>
          ))}
        </>
      );
      break;
    case 'searching':
      foundusersHTML = <p>Searching...</p>;
      break;
    case 'error':
      foundusersHTML = <p>No user found: {errMessage}</p>;
      break;
    default:
      foundusersHTML = null;
  }

  return (
    <div className="input-module bordered">
      <h1>Find User</h1>
      <form onSubmit={handleSubmit}>
        <div className="label-and-input-container">
          <label htmlFor="name">Username: </label>
          <input
            id="name"
            name="username"
            type="text"
            value={usernameToFind}
            onChange={e => setUsernameToFind(e.target.value)}
            required
          />
        </div>
        <br />
        <button type="submit" className="centered-button">
          Submit
        </button>
      </form>
      {foundusersHTML}
    </div>
  );
}

// ----- Login Component -----
export function Login() {
  const [username, setUsername] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [status, setStatus] = useState<'idle' | 'logging_in' | 'success' | 'error'>('idle');
  const [failureMessage, setFailureMessage] = useState<string | null>(null);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setStatus('logging_in');

    const loginInfo: UsernamePassword = { username, password };

    axios
      .post(`${import.meta.env.VITE_API_URL}/api/users/session`, loginInfo, { withCredentials: true })
      .then(res => setStatus('success'))
      .catch(err => {
        setStatus('error');
        setFailureMessage(err.response?.data?.message || err.code);
      });
  };

  let resultComponent;
  switch (status) {
    case 'logging_in':
      resultComponent = <p>Logging in...</p>;
      break;
    case 'error':
      resultComponent = <p>Error: {failureMessage}</p>;
      break;
    case 'success':
      resultComponent = <p>Success</p>;
      break;
    default:
      resultComponent = null;
  }

  return (
    <div className="input-module bordered">
      <h1>Login</h1>
      <form onSubmit={handleSubmit}>
        <div className="label-and-input-container">
          <label htmlFor="name">Username: </label>
          <input
            className="input-box"
            id="name"
            type="text"
            value={username}
            onChange={e => setUsername(e.target.value)}
          />
        </div>
        <div className="label-and-input-container">
          <label htmlFor="password">Password: </label>
          <input
            className="input-box"
            id="password"
            type="password"
            value={password}
            onChange={e => setPassword(e.target.value)}
          />
        </div>
        <br />
        <button type="submit" className="centered-button">
          Submit
        </button>
      </form>
      {resultComponent}
    </div>
  );
}

// ----- Logout Component -----
export function Logout() {
  const [status, setStatus] = useState<'idle' | 'logging_out' | 'successfully_logged_out' | 'failed_to_log_out'>(
    'idle'
  );

  const handleLogout = () => {
    setStatus('logging_out');
    axios
      .delete(`${import.meta.env.VITE_API_URL}/api/users/session`, { withCredentials: true })
      .then(() => setStatus('successfully_logged_out'))
      .catch(() => setStatus('failed_to_log_out'));
  };

  return (
    <div>
      <button className="centered-button" onClick={handleLogout}>
        Logout
      </button>
      <p>{status}</p>
    </div>
  );
}

// ----- LoggedInAs -----
export function LoggedInAs() {
  const [user, setUser] = useState<UserSession | null>(null);
  const [sessionRetrieved, setSessionRetrieved] = useState<'success' | 'error' | null>(null);

  useEffect(() => {
    axios
      .get<UserSession>(`${import.meta.env.VITE_API_URL}/api/users/session`, { withCredentials: true })
      .then(res => {
        setUser(res.data);
        setSessionRetrieved('success');
      })
      .catch(() => setSessionRetrieved('error'));
  }, []);

  if (sessionRetrieved === 'error') return <div>You are not logged in!</div>;
  if (sessionRetrieved === 'success' && user)
    return (
      <div>
        User: {user.username} <br />
        Email: {user.email} <br />
        User Agent: {user.user_agent}
      </div>
    );

  return null;
}

// ----- Main Export -----
export function UserCRUD() {
  return (
    <>
      <CreateUser />
      <FindAllUsers />
      <FindUser />
      <Login />
      <Logout />
      <LoggedInAs />
    </>
  );
}