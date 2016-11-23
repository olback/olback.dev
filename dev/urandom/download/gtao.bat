@echo off
title GTA: Online Solo Session Script
echo Blocking ports: 6672, 61455, 61457, 61456, 61458.
netsh advfirewall firewall set rule name="gtao" new enable=yes
timeout 20
echo Unblocking ports: 6672, 61455, 61457, 61456, 61458.
netsh advfirewall firewall set rule name="gtao" new enable=no

exit