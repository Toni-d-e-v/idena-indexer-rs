// Fetch the data from the API endpoint
fetch('http://127.0.0.1:8080/last100blocks')
  .then(response => response.json())
  .then(blocks => {
    // Get a reference to the table body
    const tableBody = document.querySelector('#blockTable tbody');

    // Loop through each block and create a table row
    blocks.forEach(block => {
      const row = document.createElement('tr');
      const tx_array2 = block.transactions.split(',');
      // remove the first element if it is empty
        if (tx_array2[0] === '') {
            tx_array2.shift();
        }
    row.innerHTML = `
    <td> <a href="block.html?block=${block.hash}">${block.height}</a></td>
    <td>${new Date(block.timestamp * 1000).toLocaleString()}</td>
    <td>${block.hash}</td>
    <td>${tx_array2.length}</td>

  `;
      // Add the row to the table
      tableBody.appendChild(row);
    });
  });

 
  function search() {
    let input = document.getElementById('search').value
    // go to block page
    window.location.href = 'block.html?block=' + input;

    // how to call
    // < form action = "javascript:search()" >
    }

    function loadMore() {
        alert('not implemented yet :(');
    }



// reload page every 1 seconds
setInterval(function(){ 
    fetch('http://127.0.0.1:8080/last100blocks')
    .then(response => response.json())
    .then(blocks => {
      // Get a reference to the table body
      const tableBody = document.querySelector('#blockTable tbody');
  
       // check our last block and compare to the last block in the table
       if (blocks[0].hash !== tableBody.rows[0].cells[2].innerHTML) {
            // reload
            location.reload();
        }


    });
}, 1000);

      

