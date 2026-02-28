import React from 'react';
import ReactDOM from 'react-dom/client';
import './styles.css';

const App = () => {
  return (
    <div className="app-container">
      <div className="sidebar">
        <h3 style={{marginBottom: '1rem'}}>QuickNote</h3>
        <button className="quick-add-btn" onClick={() => alert('Ctrl+K to add note')}>
          Quick Add (Ctrl+K)
        </button>
      </div>
      <div className="main-content">
        <input type="text" placeholder="Search notes..." className="search-bar" />
        <div className="note-list">
          <div className="note-item">Welcome to QuickNote!</div>
        </div>
      </div>
    </div>
  );
};

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
