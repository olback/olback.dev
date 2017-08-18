/*
 *   Javascript for olback.net
 */

// Print the age
// Edit this
var bYear = 2000;                        // The year you were born.
var bMonth = 09;                         // The month you were born.
var bDay = 10;                           // The day you were born.

// Don't edit this.
bMonth = bMonth - 1;                     // JS starts counting from 0. January = 0, December = 11.
var nDate = new Date();                  // Store new Date ()
var cYear = nDate.getFullYear();         // Get current year
var cMonth = nDate.getMonth();           // Get current month
var cDay = nDate.getDate();              // Get current day of the month
var age;

if(cMonth >= bMonth && cDay >= bDay){
    age = cYear - bYear;
} else {
    age = cYear - bYear - 1;
}

var myAge = document.getElementById('myAge');
myAge.innerHTML = age;

// Set year in footer
var footerYear = document.getElementById('year');
footerYear.innerHTML = cYear;

// Smooth scrolling
scrlTo = function(id) {
    document.getElementById(id).scrollIntoView({ 
        behavior: 'smooth' 
    });
}

// Change nav on scroll
var nav = 100; // Change nav-style after 100px scroll.
window.onscroll = function() {myFunction()};
function myFunction() {
	var navbar = document.getElementById("nav");
    var themecolor = document.querySelector("meta[name=theme-color]");
    
	if (document.body.scrollTop > nav || document.documentElement.scrollTop > nav) {
		navbar.className = "white";
        themecolor.setAttribute('content', 'white');

    } else {

		navbar.className = navbar.className.replace("white", "transparent");
        themecolor.setAttribute('content', '#333');

		}
}

console.log("Main.js loaded.")