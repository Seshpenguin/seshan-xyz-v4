---
title: Adding a little ARM to my G5! (Pi Zero USB Gadget on a PowerMac G5)
author: Seshan Ravikumar
type: post
date: 2020-06-10T16:09:44+00:00
url: /2020/06/10/adding-a-little-arm-to-my-g5-pi-zero-gadget-on-a-powermac-g5/
featured_image: /wp-content/uploads/2020/06/IMG_20200610_120421-scaled.jpg
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
As some of you know, the server running this website is a PowerMac G5. It&#8217;s a great little machine and I love it, but, it&#8217;s also a PowerPC machine. As much as that&#8217;s really cool, it also means that a lot of newer software doesn&#8217;t support it&#8230;

<blockquote class="wp-block-quote">
  <p>
    As it turns out, there is still plenty of support for the POWER ISA thanks to IBM still using it in servers and supercomputers, but those new POWER chips (POWER8+) are mostly ppc64le (little-endian), not big-endian like the G5 (a POWER4 CPU). Not to mention plenty of new instructions even in big-endian mode.
  </p>
</blockquote>

So what am I to do? I do have a really nice x86 server, but that&#8217;s more of a production machine (plus, I&#8217;m heading off to uni soon, I&#8217;ll be bringing the G5 with me!).

**Plug a Raspberry Pi into it!** See, I don&#8217;t want to just have a Raspberry Pi running as a separate server entirely, 1. because that&#8217;s boring, 2. because it would need to run on WiFi (which looking into the future in university, I don&#8217;t think that&#8217;ll be the greatest experience).

<div class="wp-block-image">
  <figure class="aligncenter"><img src="https://cdn.discordapp.com/attachments/642450053365235714/720307639971741766/IMG_20200610_120421.jpg" alt="https://cdn.discordapp.com/attachments/642450053365235714/720307639971741766/IMG_20200610_120421.jpg" /></figure>
</div>

For those unaware, because the **Pi Zero**&#8216;s Micro-B port has a direct connection to the SoC (with no hub in between), you can actually have it run as a USB device instead of a host. What that means is using the Linux kernel&#8217;s USB Gadget feature, the Pi Zero can make itself a USB Ethernet device (giving us a nice usb0 ethernet device on the G5)!

[Adafruit has a nice tutorial on how to set it up][1], but here&#8217;s the gist of it:

  1. Flash a fresh copy of Raspbian on an SD Card
  2. Add &#8220;`dtoverlay=dwc2`&#8221; to the bottom of config.txt (on the boot partition)
  3. Add &#8220;`modules-load=dwc2,g_ether`&#8221; after &#8220;**rootwait**&#8221; in cmdline.txt
  4. (Optionally create a empty file named &#8220;ssh&#8221; to automatically enable ssh on first boot, just do &#8220;touch ssh&#8221; in the boot partition).

After that, just plug the Pi Zero&#8217;s OTG port into your computer, and it should pop up as a network interface (on Windows you may need additional drivers)! On macOS (and Linux with avahi), your computer and the Pi will automatically get a link-local ip (169.whatever, zeroconf networking is cool), and you can just do &#8220;ssh pi@raspberrypi.local&#8221;. 

Now, you could leave it just like that and use the zeroconf network, but because I wanted the Pi Zero to be a permanent extension of my G5 server, I needed to setup a static IP, aswell as network sharing (so the Pi could access the internet).

First I added the following to &#8220;/etc/network/interfaces&#8221; on the Pi:

<pre class="wp-block-code"><code>allow-hotplug usb0
iface usb0 inet static
        address 192.168.7.2
        netmask 255.255.255.0
        network 192.168.7.0
        broadcast 192.168.7.255
        gateway 192.168.7.1</code></pre>

Then, on the G5:

<pre class="wp-block-code"><code>allow-hotplug usb0
iface usb0 inet static
        address 192.168.7.1
        netmask 255.255.255.0
        network 192.168.7.0
        broadcast 192.168.7.255</code></pre>

This gives the Pi an IP of &#8220;192.168.7.2&#8221; and the G5 &#8220;192.168.7.1&#8221;. This is nice, but we still need to have a NAT between eth0 (the ethernet port of the G5) and usb0 (the Pi Zero). [I found a nice article that explains how to do that][2], here are the steps:

  1. Uncomment `net.ipv4.ip_forward=1` in &#8220;/etc/sysctl.conf&#8221;, the do `sudo sysctl -p /etc/sysctl.conf`
  2. Run the following commands (where eth0 is the WAN port, and usb0 is the Pi&#8217;s interface):

<pre class="wp-block-code"><code>iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
iptables -A FORWARD -i eth0 -o usb0 -m state --state RELATED,ESTABLISHED -j ACCEPT
iptables -A FORWARD -i usb0 -o eth0 -j ACCEPT
</code></pre>

Your almost done, on the Pi you&#8217;ll want to disable dhcpcd (since we have a static IP), and edit &#8220;/etc/resolv.conf&#8221; to use some public DNS server (like 1.1.1.1). And that&#8217;s it! The Pi Zero should be able to access the internet.

**Still one more tweak to go though.** You can SSH into the Pi (or access any service) locally on the machine using &#8220;192.168.7.2&#8221;, but the Pi isn&#8217;t exposed outside of the server. If I was running, say, web services, I&#8217;d use NGINX as a reverse proxy. But for things like SSH, you need to basically port forward. Most people would add some additional iptables entries, but since I was messing around with Golang a bit while doing this project, I decided I&#8217;d write a simple TCP proxy to forward certain ports of the server to the Pi.

[You can find that code on my GitHub][3], and basically it just parses a config file with a list of ports to forward. This way, services on the Pi appear alongside everything on the server, it&#8217;s great! Basically &#8220;ssh seshpenguin@192.168.0.134&#8221; goes to the host OS, and &#8220;ssh pi@192.168.0.134 -p 2287&#8221; goes to the Pi. ?

<div class="wp-block-image">
  <figure class="aligncenter"><img src="https://cdn.discordapp.com/attachments/642450053365235714/720307641343148102/IMG_20200610_120443.jpg" alt="https://cdn.discordapp.com/attachments/642450053365235714/720307641343148102/IMG_20200610_120443.jpg" /></figure>
</div>

And there you go! The SeshanXYZ server now has a little Pi companion! It works great, and I can run stuff that would otherwise not work on the G5 (like modern versions of NodeJS). That being said, a Pi zero isn&#8217;t the fastest thing in the world&#8230; but it&#8217;s the only Pi that can run in Gadget mode&#8230; **or is it?** As it turns out, the Pi 4&#8217;s USB-C port can actually do that same thing! Yes that&#8217;s right ladies and gentlemen, you can get this same experience but with 64-bit quad-core goodness and up to 8GB of RAM! I might want to get my hands on a Raspberry Pi 4&#8230;

 [1]: https://learn.adafruit.com/turning-your-raspberry-pi-zero-into-a-usb-gadget/ethernet-gadget
 [2]: https://makandracards.com/makandra/46300-ubuntu-share-internet-connections-with-other-computers
 [3]: https://github.com/Seshpenguin/xyz-proxy