/*
 *  olback.net v3.0
 */

document.addEventListener('DOMContentLoaded', function () {

    var footer = document.getElementsByTagName('footer')[0];

    document.addEventListener('scroll', function() {

        if(window.scrollY > footer.scrollHeight + 50 || document.documentElement.scrollTop > footer.scrollHeight + 50) {
            footer.style.visibility = 'visible';
        } else {
            footer.style.visibility = 'hidden';
        }

    }, { passive: true });

    console.log('Loaded');

}, { passive: true });
