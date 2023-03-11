// Import basic react state
import React from 'react';
import '../App.css';
import Countdown from 'react-countdown';


// sarch
import Search from './Search.js';

const Home = () => {
    const [url, setUrl] = React.useState('http://127.0.0.1:8080');
    const [blocks, setBlocks] = React.useState([]);
    const [firstLoad, setFirstLoad] = React.useState(true);
    const [lastBlock, setLastBlock] = React.useState({});
    const [epoch, setEpoch] = React.useState(0);
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
        fetch_epoch();
    };
    // http://127.0.0.1:8080/epoch
    const fetch_epoch = () => {
        fetch(url + '/epoch')
        .then(response => response.json())
        .then(epoch => {
            setEpoch(epoch);
        });
    };
   
    if (firstLoad) {
        fetch_last100();
        fetch_epoch();
        
        setFirstLoad(false);
    }
    // timer for epoch.nextValidation



    // fetch 100 every 2.5 seconds
    setInterval(fetch_last100, 2500);

        


    return (
        <div>
            <Search />
         

            <small>
                Fast, simple, blockchain explorer for Idena. 
            </small>
    
            <br />
            <a href="https://github.com/Toni-d-e-v/idena-indexer-rs">Github</a>

            <br />
            <div class="content">
                Next validation
                        <p>
                            {
                                epoch.nextValidation === NaN ? <span> Loading... </span> : 
                                <Countdown 
                                
                                date={epoch.nextValidation}
                                // days/hours/minutes/seconds/ms
                                renderer={({ days, hours, minutes, seconds, completed }) => {
                                    if (completed) {
                                        // Render a completed state
                                        return <span>Validation time!</span>;
                                    } else {
                                        // Render a countdown
                                        return <span>{days} days, {hours} hours, {minutes} minutes, {seconds} seconds</span>;
                                    }
                                }}
                                />
                            }
                        </p>
                    </div>
            <br />
            <article>
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
            </article>

        </div>

    );
};
  
export default Home;
  