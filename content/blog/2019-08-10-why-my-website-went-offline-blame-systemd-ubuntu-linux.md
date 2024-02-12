---
title: Why My Website Went Offline (Blame systemd? Ubuntu? Linux?)
author: Seshan Ravikumar
type: post
date: 2019-08-10T23:58:09+00:00
url: /2019/08/10/why-my-website-went-offline-blame-systemd-ubuntu-linux/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
<figure class="wp-block-image"><img loading="lazy" width="1024" height="658" src="https://seshan.xyz/wp-content/uploads/2019/08/IMG_20190810_1945052-1024x658.jpg" alt="" class="wp-image-200" srcset="https://seshan.xyz/wp-content/uploads/2019/08/IMG_20190810_1945052-1024x658.jpg 1024w, https://seshan.xyz/wp-content/uploads/2019/08/IMG_20190810_1945052-300x193.jpg 300w, https://seshan.xyz/wp-content/uploads/2019/08/IMG_20190810_1945052-768x494.jpg 768w" sizes="(max-width: 1024px) 100vw, 1024px" /></figure> 

Hey Guys! You may have noticed that my website has been offline for a few days now. Well, good news, it&#8217;s back! (duh.)

Now admittedly the multi-day downtime was mostly down to my laziness&#8230; but we can ignore that for now&#8230; (I kinda put off looking at why the website went off)



So what went wrong? Well, after a small brownout, the PowerMac G5 (as expected) restarted. The problem? The network interface wasn&#8217;t being brought up.

As it turns out, there seems to be a bug with networking.service (or atleast, whatever service it calls) on Ubuntu 16.04 where it&#8217;s for some reason unable to automatically detect the network interface name.

I remedied this way back when I initially install Ubuntu by manually adding the interface name into &#8220;cat /etc/network/interfaces&#8221; (you can find the name from &#8220;ip addr&#8221;):

<pre class="wp-block-code"><code># The primary network interface
auto enP48240p4s15f0
iface enP48240p4s15f0 inet dhcp</code></pre>

This worked pretty well up until now: when systemd/udev decided that this time we boot, the interface name should be different. So much for &#8220;Predictable Network Interface Names&#8221;&#8230;

Now this obviously isn&#8217;t a wide spread issues, and I&#8217;m not totally surprised that this relativity obscure system (a PowerPC machine with it&#8217;s GEM ethernet) has some issues. In any case, if you have a PowerMac, keep an eye out for network interface name changes!