// Import basic react state
import React from 'react';
import '../App.css';

const Account = () => {
    const [url, setUrl] = React.useState('http://127.0.0.1:8080');

    const Account1 = window.location.href.split('/account/')[1];
    const [AccountData, setAccountData] = React.useState({});
    const [txData, setTxData] = React.useState([]);
    const [firstLoad, setFirstLoad] = React.useState(true);


    const fetch_account = () => {
        fetch(url + '/account/' + Account1)
        .then(response => response.json())
        .then(account => {
           // {"address": "0x2047035e7d34bf806ee2fdb238e2aad91379ec02", "balance": {"balance":"9.8613837208454578","mempoolNonce":560,"nonce":560,"replenishedStake":"0","stake":"0"}, "txs": ["0x78eeb21eaa44925703e864af27327d68caed20e2ffd74950b05920b4bcde60d1"]}
           for (let i = 0; i < account.txs.length; i++) {
               fetch(url + '/tx/' + account.txs[i])
               .then(response => response.json())
               .then(tx => {
                   setTxData(txData => [...txData, tx]);
               });
           }
           setAccountData(account);
           console.log(account);
        });

    }
    if (firstLoad) {
        fetch_account();
        setFirstLoad(false);
    }


    return (
        <div>
            <h1>Idena Block Explorer - Account <a href="/">Home</a></h1> 
            <h3>{AccountData.address}</h3>
            { AccountData.balance ? <h3> Balance: {AccountData.balance.balance} </h3> : null }
            { AccountData.balance ? <h3> Stake: {AccountData.balance.stake} </h3> : null }
            { AccountData.balance ? <h3> Replenished Stake: {AccountData.balance.replenishedStake} </h3> : null }
            { AccountData.balance ? <h3> Nonce: {AccountData.balance.nonce} </h3> : null }
  


            <h2>Transactions</h2>
            <small> Transactions that are in our indexer database. </small>
            <table id="txTable">
                <tbody>
                {
                        txData.length === 0 ? <tr><td>No transactions</td></tr> : 
                        <tr>
                        <td>Hash</td>
                        <td>From</td>
                        <td>To</td>
                        <td>Value</td>

                    </tr>
                        
                    }
                    {txData.map(tx => (
                        <tr>
                            <td><a href={'/tx/' + tx.hash}>{tx.hash.substring(0, 10) + '...'}</a></td>
                            <td><a href={'/account/' + tx.from}>{tx.from.substring(0, 10) + '...'}</a></td>
                            <td><a href={'/account/' + tx.to}>{tx.to.substring(0, 10) + '...'}</a></td>
                            <td>{tx.amount}</td>

                           

                        </tr>
                    ))}
                </tbody>
                            
           
       


               
            </table>

                
       


            



            


        </div>

    );
};
  
export default Account;
  