#!/bin/bash
set -x

# NOTE: we dont set LAN IP on boosters for now, that means they will stay the same as in the docs

# set sound volume to 100%
su booster -c "pactl -- set-sink-volume 0 100%"

# play sound
su booster -c "/usr/bin/paplay /home/booster/naoth/Media/usb_start.wav"

HEAD_ID=$(sed -n '/^Nickname:/ { s/^Nickname:[[:space:]]*//; s/ /_/g; p }' /opt/booster/robot_info.txt)
N=$(grep "$HEAD_ID" /opt/naoth/robots.cfg | sed 's/.*=\([0-9]\+\)/\1/')
if [[ -z "$N" ]]; then N=99; fi # fallback number 99 !

echo "Generate network configuration";
WLAN_SSID="NAONET(5G)"
WLAN_PW="a1b0a1b0a1"
WLAN_IP="10.0.4"
WLAN_SUBNET_MASK="16"
# TODO make DNS configurable (needed for HU networks)

# e.g. for a competition set the following
# WLAN_SSID="SPL_A"
# WLAN_PW="Nao?!Nao?!"

sudo tee /etc/NetworkManager/system-connections/BU-WIFI.nmconnection > /dev/null <<EOF
[connection]
id=BU-WIFI
type=wifi
autoconnect-priority=1
[match]
interface-name=wl*

[wifi]
mode=infrastructure
ssid=$WLAN_SSID
powersave=2

[wifi-security]
auth-alg=open
key-mgmt=wpa-psk
psk=$WLAN_PW

[ipv4]
method=manual
addresses=$WLAN_IP.$N/$WLAN_SUBNET_MASK
gateway=$WLAN_IP.1
dns=8.8.8.8;141.20.26.7;

[ipv6]
addr-gen-mode=stable-privacy
method=auto

[proxy]
EOF

sudo chmod 600 /etc/NetworkManager/system-connections/BU-WIFI.nmconnection

sudo nmcli connection reload
# sudo nmcli device disconnect wlP1p1s0 # if we encounter errors we might need to do this
sudo nmcli connection up BU-WIFI
