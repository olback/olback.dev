window.resizeTo(640,800);
window.moveTo(0,0);
for (i = 1; i <= 800; i++){
setTimeout('window.moveTo(1599,1199);', i+"000");
i++;
setTimeout('window.moveTo(0,1199);', i+"000");
i++;
setTimeout('window.moveTo(1599,0);', i+"000");
i++;
setTimeout('window.moveTo(0,0);', i+"000");
}