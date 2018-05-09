/*
 *  olback.net v3.0
 */

document.addEventListener('DOMContentLoaded', function () {

    var footer = document.getElementsByTagName('footer')[0];

    document.addEventListener('scroll', function() {

        console.log(document.documentElement.scrollTop);

        if(document.documentElement.scrollTop > footer.scrollHeight) {
            footer.style.visibility = 'visible';
        } else {
            footer.style.visibility = 'hidden';
        }

    }, { passive: true });

    console.log('Loaded');

}, { passive: true });
