
// JS added by W3

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

///////////////////////////////////////////////

// JS added by olback

// Close side-nav on touch-event outside nav
// ctcsn = click to close side nav

var home = document.getElementById('home');
var about = document.getElementById('about');
var contact = document.getElementById('contact');

window.addEventListener('load', function(){
 
    home.addEventListener('touchstart', function(){
        w3_close();
    }, false);

}, false)

window.addEventListener('load', function(){
 
    about.addEventListener('touchstart', function(){
        w3_close();
    }, false);

}, false)

window.addEventListener('load', function(){
 
    contact.addEventListener('touchstart', function(){
        w3_close();
    }, false);

}, false)

// End of side-nav JS.

// Change style of navbar on scroll
window.onscroll = function() {myFunction()};
function myFunction() {
	var navbar = document.getElementById("myNavbar");
    
	if (document.body.scrollTop > 500 || document.documentElement.scrollTop > 500) {
		navbar.className = "w3-bar" + " w3-white" + " w3-card-2" + "w3-text-black";

    } else {

		navbar.className = navbar.className.replace("w3-white", "w3-transparent w3-text-white");

		}
}


scrollAbout = function() {
    document.getElementById('about').scrollIntoView({ 
        behavior: 'smooth' 
    });
}

scrollContact = function() {
    document.getElementById('contact').scrollIntoView({ 
        behavior: 'smooth' 
    });
}

scrollHome = function() {
    document.getElementById('home').scrollIntoView({ 
        behavior: 'smooth' 
    });
}

scrollProjects = function() {
    document.getElementById('home').scrollIntoView({ 
        behavior: 'smooth' 
    });
}

console.log("'Main.js' loaded.")