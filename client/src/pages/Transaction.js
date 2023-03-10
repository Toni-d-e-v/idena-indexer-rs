// Import basic react state
import React from 'react';
import '../App.css';

const Transaction = () => {
    const [url, setUrl] = React.useState('http://127.0.0.1:8080');

    const tx1 = window.location.href.split('/tx/')[1];

    const [txData, setTxData] = React.useState({});
    const [firstLoad, setFirstLoad] = React.useState(true);


    const fetch_tx = () => {
        fetch(url + '/tx/' + tx1)
        .then(response => response.json())
        .then(tx => {
            setTxData(tx);
        });
    }
    if (firstLoad) {
        fetch_tx();
        setFirstLoad(false);
    }


    return (
        <div>
            <h1>Idena Block Explorer - Transaction <a href="/">Home</a></h1> 
            <h3>{txData.hash}</h3>
            <h3> Epoch: {txData.epoch} </h3>
            <table>
                <tbody>
                    <tr>
                        <td>From:</td>
                        <td><a href={'/account/' + txData.from }>{txData.from}</a></td>
                    </tr>
                    <tr>
                        <td>To:</td>
                        <td><a href={'/account/' + txData.to }>{txData.to}</a></td>
                    </tr>
                    <tr>
                        <td>Amount:</td>
                        <td>{txData.amount}</td>
                    </tr>
                    <tr>
                        <td>Fee:</td>

                        <td>{txData.fee}</td>
                    </tr>
                    <tr>
                        <td>Nonce:</td>
                        <td>{txData.nonce}</td>
                    </tr>
                    <tr>
                        <td>Type:</td>
                        <td>{txData.type}</td>
                    </tr>
                    <tr>
                        <td>Payload:</td>
                        <td>{txData.payload}</td>
                    </tr>
                    <tr>
                        <td>Timestamp:</td>
                        <td>{txData.timestamp}</td>
                    </tr>
                    <tr>
                        <td>Block:</td>
                        <td><a href={'/block/' + txData.block_h }>{txData.block_h}</a></td>
                    </tr>



                </tbody>
            </table>
            

                
       


            



            


        </div>

    );
};
  
export default Transaction;
  