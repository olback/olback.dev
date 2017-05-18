
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
    
	if (document.body.scrollTop > 500 || document.documentElement.scrollTop > 500) {
		navbar.className = "w3-bar" + " w3-white" + " w3-card-2" + "w3-text-black";

    } else {

		navbar.className = navbar.className.replace("w3-white", "w3-transparent w3-text-white");

		}
}

// Smooth scrolling
scrollHome = function() {
    document.getElementById('home').scrollIntoView({ 
        behavior: 'smooth' 
    });
}

scrollAbout = function() {
    document.getElementById('about').scrollIntoView({ 
        behavior: 'smooth' 
    });
}

scrollProjects = function() {
    document.getElementById('projects').scrollIntoView({ 
        behavior: 'smooth' 
    });
}

scrollContact = function() {
    document.getElementById('contact').scrollIntoView({ 
        behavior: 'smooth' 
    });
}

console.log("'Main.js' loaded.");
