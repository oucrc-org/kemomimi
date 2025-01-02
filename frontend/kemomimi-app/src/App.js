import React from 'react';
import PublicItemsList from './components/PublicItemsList';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1>KEMOMIMI SYSTEM</h1>
      </header>
      <main>
        <PublicItemsList />
      </main>
    </div>
  );
}

export default App;