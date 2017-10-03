/*
 *   Javascript for olback.net
 */


const year = 2000;                        // The year you were born.
const bMonth = 9;                         // The month you were born.
const day = 10;                           // The day you were born.

/* ------------------------------------------------------------------------------ */

const date = new Date();                  // Store new Date ()
const cYear = date.getFullYear();         // Get current year
const cMonth = date.getMonth();           // Get current month
const cDay = date.getDate();              // Get current day of the month
const month = bMonth - 1;                 // JS starts counting from 0. January = 0, December = 11.
let age;


if(cMonth >= month && cDay >= day){
  age = cYear - year;
} else if(cMonth > month) {
  age = cYear - year;
} else {
  age = cYear - year - 1
}

const myAge = document.getElementById('myAge');
myAge.innerHTML = age;

// Set year in footer
const footerYear = document.getElementById('year');
footerYear.innerHTML = cYear;

// Smooth scrolling
scrlTo = function(id) {
    document.getElementById(id).scrollIntoView({ 
        behavior: 'smooth' 
    });
}

// Change nav on scroll
const nav = 100; // Change nav-style after 100px scroll.
window.onscroll = function() {myFunction()};
function myFunction() {
	const navbar = document.getElementById("nav");
    const themecolor = document.querySelector("meta[name=theme-color]");
    
	if (document.body.scrollTop > nav || document.documentElement.scrollTop > nav) {
		navbar.className = "white";
        themecolor.setAttribute('content', 'white');

    } else {

		navbar.className = navbar.className.replace("white", "transparent");
        themecolor.setAttribute('content', '#333');

	}
}

// Enable contact form send button
document.getElementById("cformButton").disabled = false;

console.log("Main.js loaded.")
