// Import basic react state
import React from 'react';
import '../App.css';



const Home = () => {
    const [url, setUrl] = React.useState('http://127.0.0.1:8080');
    const [blocks, setBlocks] = React.useState([]);
    const [firstLoad, setFirstLoad] = React.useState(true);
    const [lastBlock, setLastBlock] = React.useState({});
    const fetch_last100 = () => {
        fetch(url + '/last100blocks')
        .then(response => response.json())
        .then(blocks => {
            console.log(blocks);
            blocks.forEach(block => {
                const tx_array = block.transactions.split(',');
                if (tx_array[0] === '') {
                    tx_array.shift();
                }
                block.transactions = tx_array;
            });

            setBlocks(blocks);
        });
    }
    if (firstLoad) {
        fetch_last100();
        setFirstLoad(false);
    }

    // fetch 100 every 2.5 seconds
    setInterval(fetch_last100, 2500);

        


    return (
        <div>
            <nav>
                <h1>Idena Blockchain Explorer</h1>
                <input type="text" placeholder="Search for a block or transaction" />

            </nav>
            <small>
                Fast, simple, blockchain explorer for Idena. 
                <br />
                <strong>Built with Rust and React.</strong>
            </small>
            <br />

            <small>Donate: 0xa15de4839ed11ac66a6ff0a4e58fe90d99e67b3d  </small>
            <br />
            <a href="https://github.com/Toni-d-e-v/idena-indexer-rs">Github</a>

            <br />
            
            <br />

            <table id="blockTable">
                <thead>
                    <tr>
                        <th>Height</th>
                        <th>Hash</th>
                        <th>Transactions</th>

                    </tr>
                </thead>
                <tbody>
                    {blocks.map(block => (
                        <tr key={block.hash}>
                            <td><a href ={'/block/' + block.hash} >{block.height}</a></td>
                            <td>{block.hash.substring(0, 35)}...</td>
                            <td>{block.transactions.length}</td>

                        </tr>
                    ))}
                </tbody>
            </table>
        </div>

    );
};
  
export default Home;
  