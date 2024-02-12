---
title: The IoT Christmas Tree ft. Ubuntu Core
author: Seshan Ravikumar
type: post
date: 2018-12-08T19:20:06+00:00
url: /2018/12/08/the-iot-christmas-tree-ft-ubuntu-core/
categories:
  - Uncategorized

---
It&#8217;s that time of year&#8230; and so the Christmas Tree makes a return once again.  
<figure class="wp-block-image">

![][1] </figure> 

Notice anything interesting? Let&#8217;s take a closer look&#8230;  
<figure class="wp-block-image">

![][2] </figure> 

It&#8217;s a Raspberry Pi! Okay, I&#8217;ll just say it right now, the tree has nothing to do with the Raspberry Pi.  
If we look under the tree:  
<figure class="wp-block-image">

![][3] </figure> 

This Raspberry Pi is running Ubuntu Core with the chromium-mir-kiosk snap. What is Ubuntu Core you ask? It&#8217;s actually pretty neat.  
Basically Ubuntu Core is a minimal version of Ubuntu designed for IoT and Embedded applications. The whole system is based around snaps (aka. Snappy). A snap is a little self contained application bundle, that runs isolated (aka in a container) from the rest of the system. If you use a recent version of Ubuntu, then some of your applications are already bundled as snaps!  
Unlike standard Ubuntu though, the whole system is snaps, including the kernel. Also, the rootfs is read-only. All of this makes Ubuntu Core really well suited for embedded applications. In my case, I have Ubuntu Core running the web kiosk demo. Basically it&#8217;s just Chromium running in a snap, displaying a countdown until Christmas.  
Another interesting feature of snaps is interfaces. Basically, by default a snap has no connections to the rest of the system, and is completely isolated. Interfaces allow snaps to communicate with each other using a concept of plugs and slots. For example, the web kiosk demo has two main slots: a Wayland slot and PulseAudio slot. The Wayland slot is &#8220;plugged into&#8221; by the mir-kiosk snap, which is running as a Wayland display server. The PulseAudio slot is plugged into by, well, the PulseAudio snap, providing audio.  
  
So yea, Ubuntu Core and snaps, two pretty cool technologies. Ubuntu Core seems to be a good fit for all sorts of IoT and Embedded applications (much better than say Windows :P). 

**Also!** If you guys want a super-fancy (not really) interface for your own IoT Tree, you can check mine out [here.][4]

 [1]: http://legacy.seshan.xyz/flow/files/WP_20181208_22_07_14_Rich.jpg
 [2]: http://legacy.seshan.xyz/flow/files/WP_20181208_21_01_24_Rich.jpg
 [3]: http://legacy.seshan.xyz/flow/files/WP_20181209_13_13_53_Rich.jpg
 [4]: https://seshan.xyz/legacy/flow/iotxmas/