/* JavaScript for olback.net */

// Vars:
var nav = 100; // Change nav-style after 100px scroll.

// Toggle between showing and hiding the sidebar when clicking the menu icon
var mySidebar = document.getElementById("mySidebar");

function w3_open() {
    if (mySidebar.style.display === 'block') {
        mySidebar.style.display = 'none';
    } else {
        mySidebar.style.display = 'block';
    }
}

// Close the sidebar with the close button
function w3_close() {
    mySidebar.style.display = "none";
}

// Close side-nav on touch-event outside nav
var close = document.getElementById('close');

window.addEventListener('load', function(){
 
    close.addEventListener('touchstart', function(){
        w3_close();
    }, false);

}, false)

// Change style of navbar on scroll
window.onscroll = function() {myFunction()};
function myFunction() {
	var navbar = document.getElementById("myNavbar");
    var tc = document.querySelector("meta[name=theme-color]");
    
	if (document.body.scrollTop > nav || document.documentElement.scrollTop > nav) {
		navbar.className = "w3-bar" + " white-nav" + " w3-card-2" + " w3-text-black";
        tc.setAttribute('content', 'white');

    } else {

		navbar.className = navbar.className.replace("white-nav", "w3-transparent w3-text-white");
	    navbar.className = navbar.className.replace("w3-text-black", "");
        tc.setAttribute('content', '#333');

		}
}

// Smooth scrolling
scrlTo = function(id) {
    document.getElementById(id).scrollIntoView({ 
        behavior: 'smooth' 
    });
}
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

console.log("'Main.js' loaded.");
console.log('Found any problems or security flaws with this page? Report them to web-olback.net@olback.net.');