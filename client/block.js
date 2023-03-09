// get ?block 
const params = new Proxy(new URLSearchParams(window.location.search), {
    get: (searchParams, prop) => searchParams.get(prop),
  });
  // Get the value of "some_key" in eg "https://example.com/?some_key=some_value"
  
let block = params.block;

if (block) {
  // check if int
    if (block.match(/^\d+$/)) {
        // http://localhost:8080/block/height/ + 
        fetch('http://localhost:8080/block/height/' + block)
        .then(response => response.json())
        .then(block => {
            document.getElementById('blockHeight').innerHTML = block.height;
            document.getElementById('blockHash').innerHTML = block.hash;
            document.getElementById('blockTimestamp').innerHTML = new Date(block.timestamp * 1000).toLocaleString();
            document.getElementById('blockCoinbase').innerHTML = block.coinbase;

            const tx_array = block.transactions.split(',');
            // add to table
            const tableBody = document.querySelector('#txTable tbody');
            tx_array.forEach(tx => {
                const row = document.createElement('tr');
                row.innerHTML = `
                <td> <a href="transaction.html?tx=${tx}">${tx}</a></td>
                `;
                tableBody.appendChild(row);
            }
            );
            

        }
        );
    } else {
        // http://localhost:8080/block/height/ + 
        fetch('http://localhost:8080/block/' + block)
        .then(response => response.json())
        .then(block => {
            document.getElementById('blockHeight').innerHTML = block.height;
            document.getElementById('blockHash').innerHTML = block.hash;
            document.getElementById('blockTimestamp').innerHTML = new Date(block.timestamp * 1000).toLocaleString();
            // blockCoinbase + image of robohash / make image non selectable with style
            document.getElementById('blockCoinbase').innerHTML = block.coinbase + '<img style="user-select: none;" src="https://robohash.org/' + block.coinbase + '?size=50x50" alt="robohash" />';
            const tx_array = block.transactions.split(',');
            const tableBody = document.querySelector('#txTable tbody');

            tx_array.forEach(tx => {
                const row = document.createElement('tr');
                row.innerHTML = `
                <td> <a href="transaction.html?tx=${tx}">${tx}</a></td>
                `;
                tableBody.appendChild(row);
                console.log(tx_array);

            }

            );


            

        }
        );

    }
} else {
    res.render('block', {block: null});
}



