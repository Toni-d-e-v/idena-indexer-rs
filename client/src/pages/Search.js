import React from 'react';
import '../App.css';

// search
const Search = () => {
    const [theme , setTheme] = React.useState('light');
    const [firstLoad, setFirstLoad] = React.useState(true);
    const theme_switch = () => {
        const body = document.querySelector('html');
        const theme = body.getAttribute('data-theme');
        if (theme === 'dark') {
            body.setAttribute('data-theme', 'light');
            localStorage.setItem('theme', 'light');
        } else {
            body.setAttribute('data-theme', 'dark');
            localStorage.setItem('theme', 'dark');
        }
        setTheme(localStorage.getItem('theme'));
        
    }
    if (firstLoad) {
        setTheme(localStorage.getItem('theme'));
     

        setFirstLoad(false);
    }
    const search = () => {
        const search = document.querySelector('input').value;
        if (search.length === 66) {
            window.location.href = '/tx/' + search;
        }
        
        if (search.length === 42) {
            window.location.href = '/account/' + search;
        }
        // if num
        if ( search.match(/^[0-9]+$/) ) {
            window.location.href = '/block/' + search;
        }
    }
    return (
        <nav>
        <ul>
            <li><strong><a href="/">Idena Explorer <small>RS</small></a></strong></li>
        </ul>
        <ul>
 
            <li><input type="text" id="search" placeholder="block, address, transaction" /></li>
            <li><button onClick={search}>Search</button></li>
            <li><button onClick={theme_switch}> 
                {
                    theme === 'dark' ? '🌞' : '🌙'
                }
            </button></li>
        </ul>
        </nav>
    )
}




export default Search;
