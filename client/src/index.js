import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import reportWebVitals from './reportWebVitals';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Block from "./pages/Block";
import Transaction from "./pages/Transaction";
import Account from "./pages/Account";
// pico css
import '@picocss/pico';
export default function App() {
     // theme
     const body = document.querySelector('html');
     const theme = localStorage.getItem('theme');
     if (theme === 'dark') {
         body.setAttribute('data-theme', 'dark');
     } else {
         body.setAttribute('data-theme', 'light');
     }
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" >
          <Route index element={<Home />} />
        </Route>
        {/* block/:block */}
        <Route path="block/:block"  >
          <Route index element={<Block  />} />
        </Route>
        {/* tx/:tx */}
        <Route path="tx/:tx"  >
          <Route index element={<Transaction  />} />
        </Route>
        <Route path="account/:account"  >
          <Route index element={<Account  />} />
        </Route>
        

         
        
      </Routes>
    </BrowserRouter>
  );
}

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);