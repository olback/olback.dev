/*
 *   Javascript for olback.net
 */


var bYear = 2000;                        // The year you were born.
var bMonth = 09;                         // The month you were born.
var bDay = 10;                           // The day you were born.
const firefoxVersion = 53;
const chromeVersion = 58;
const ieVersion = 9;
const edgeVersion = 13;
const safariVersion = 9;
const operaVersion = 45;
var updateBrowser = document.getElementById('updateBrowser');

/* ------------------------------------------------------------------------------------------------------ */
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

// Enable contact form send button
document.getElementById("cformButton").disabled = false;

// Warn if using old browser
navigator.sayswho = (function(){
    var ua = navigator.userAgent, tem, 
    M = ua.match(/(opera|chrome|safari|firefox|msie|trident(?=\/))\/?\s*(\d+)/i) || [];
    if(/trident/i.test(M[1])){
        tem =  /\brv[ :]+(\d+)/g.exec(ua) || [];
        return 'IE '+(tem[1] || '');
    }
    if(M[1] === 'Chrome'){
        tem = ua.match(/\b(OPR|Edge)\/(\d+)/);
        if(tem != null) return tem.slice(1).join(' ').replace('OPR', 'Opera');
    }
    M = M[2]? [M[1], M[2]]: [navigator.appName, navigator.appVersion, '-?'];
    if((tem = ua.match(/version\/(\d+)/i)) != null) M.splice(1, 1, tem[1]);
    return M.join(' ');
})();

var browser = navigator.sayswho.split(" ");

browser.name = function(){
    return browser[0];
}();

browser.version = function(){
    return browser[1];
}();

oldBrowser = function() { // Use Switch instead? no?

    if(browser.name == "Firefox" && browser.version > firefoxVersion) {
        return false;
    } else if(browser.name == "Chrome" && browser.version > chromeVersion) {
        return false;
    } else if(browser.name == "IE" && browser.version > ieVersion) {
        return false;
    } else if(browser.name == "Edge" && browser.version > edgeVersion) {
        return false;
    } else if (browser.name == "Safari" && browser.version > safariVersion) {
        return false;
    } else if(browser.name == "Opera" && browser.version > operaVersion) {
        return false;
    } else {
        return true;
    }

}();

browserAlert = function() {

    if (document.cookie == "showBrowserAlert=false") {
        return false;
    } else {
        return true;
    }

}();

closeModalBU = function() {
    updateBrowser.hidden = true;
    document.cookie = "showBrowserAlert=false; expires=Fri, 31 Dec 9999 23:59:59 GMT; path=/;";
};

if(oldBrowser && browserAlert) {
    updateBrowser.hidden = false;
}

console.log("Main.js loaded.")