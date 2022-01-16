import logo from './logo.svg';
import React from "react";
import './App.css';
import { gql, useQuery } from '@apollo/client';

const GET_ACCOUNTS = gql`
query name {
  accounts {
    id
    email
    login
  }
}`;

function FetchData() {
  const { loading, error, data } = useQuery(GET_ACCOUNTS)
  if (loading) { return <p>Loadingu</p> }
  if (error) { return <p>Erroru</p> }

  //console.log(data.accounts[0].email)
  return (
    <>
      <ol>
      {data.accounts.map((element, key) => (
        <li key={key}>{element.email}</li>

      ))}
      </ol>
    </>
  );
}

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <FetchData />
      </header>
    </div>
  );
}

export default App;