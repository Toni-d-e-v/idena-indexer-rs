// Import basic react state
import React from 'react';
import '../App.css';

const Block = () => {
    const [url, setUrl] = React.useState('http://127.0.0.1:8080');
    // /block/:block
    // get block by hash or height
    // get block from window.location.href
    const block1 = window.location.href.split('/block/')[1];

    const [blockData, setBlockData] = React.useState({});
    const [firstLoad, setFirstLoad] = React.useState(true);
    const [transactions, setTransactions] = React.useState([]);
    const fetch_block = () => {
        // if starts with 0x, then it's a hash
        if (block1.startsWith('0x')) {
            fetch(url + '/block/' + block1)
            .then(response => response.json())
            .then(block => {
                const tx_array = block.transactions.split(',');
                if (tx_array[0] === '') {
                    tx_array.shift();
                }
                block.transactions = tx_array;
                for (let i = 0; i < block.transactions.length; i++) {
                    console.log(block.transactions[i]);
                    fetch(url + '/tx/' + block.transactions[i])
                    .then(response => response.json())
                    .then(tx => {
                        setTransactions(transactions => [...transactions, tx]);
                    });
                }
                console.log(transactions);

                setBlockData(block);
            });
        } else {
            fetch(url + '/block/height/' + block1)
            .then(response => response.json())
            .then(block => {
                const tx_array = block.transactions.split(',');
                if (tx_array[0] === '') {
                    tx_array.shift();
                }
                block.transactions = tx_array;
                for (let i = 0; i < block.transactions.length; i++) {
                    console.log(block.transactions[i]);
                    fetch(url + '/tx/' + block.transactions[i])
                    .then(response => response.json())
                    .then(tx => {
                        setTransactions(transactions => [...transactions, tx]);
                    });
                    console.log(transactions);


                }
                setBlockData(block);
            });
        }
  
     



        
        
    }
    if (firstLoad) {
        fetch_block();


        setFirstLoad(false);
    }


    return (
        <div>
            <h1>Idena Block Explorer - Block <a href="/">Home</a></h1> 
            <h2>Block {blockData.height}</h2>
            <table id="blockTable">
                <tbody>
                    <tr>
                        <td>Hash:</td>
                        <td>{blockData.hash}</td>
                    </tr>
                    <tr>
                        <td>Height:</td>
                        <td>{blockData.height}</td>
                    </tr>
                    <tr>
                        <td>Parent Hash:</td>
                        <td>{blockData.parentHash}</td>
                    </tr>
                    <tr>
                        <td>Timestamp:</td>
                        <td>{blockData.timestamp}</td>
                    </tr>
                    
                    <tr>
                        <td>Block Producer: </td>
                        <td><a href={'/account/' + blockData.coinbase}>{blockData.coinbase}</a></td>
                    </tr>

                </tbody>
                
            </table>
            <h2>Transactions</h2>
            <table id="txTable">
                <tbody>
                    {
                        transactions.length === 0 ? <tr><td>No transactions</td></tr> : 
                        <tr>
                        <td>Hash</td>
                        <td>From</td>
                        <td>To</td>
                        <td>Value</td>

                    </tr>
                        
                    }
                    {transactions.map(tx => (
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
  
export default Block;
  