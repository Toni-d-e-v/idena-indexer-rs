import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import reportWebVitals from './reportWebVitals';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Home from "./pages/Home";
import Block from "./pages/Block";
import Transaction from "./pages/Transaction";
import Account from "./pages/Account";
export default function App() {
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