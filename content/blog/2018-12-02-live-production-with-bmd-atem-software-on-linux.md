---
title: Live Production with BMD ATEM Software on Linux
author: Seshan Ravikumar
type: post
date: 2018-12-02T19:14:19+00:00
url: /2018/12/02/live-production-with-bmd-atem-software-on-linux/
categories:
  - Uncategorized

---
Recently my school&#8217;s band had a fall showcase, and being a part of the school&#8217;s ComTech club, it was our job to film and record the whole thing. 

Our setup is actually pretty cool. The centerpiece is got to be the Blackmagic Design ATEM Production Studio 4K, an awesome 4k-ready switcher. For this setup, we had two cameras connected using SDI, and a handful of lower third graphics uploaded to the switcher. The real fun begins is when we begin to control the switcher&#8230; <figure class="wp-block-image">

![][1] </figure> 

Notice anything interesting? _It&#8217;s the ATEM Software Control running on Linux!_ For those curious, that&#8217;s my laptop that I take around (MacBook Pro, Early 2008 with a Core 2 Duo and 4GB of RAM). At the time of writing it is running Fedora 29 with MATE.  
Whats probably more exciting is the ATEM Software is running on WINE! Although a native Linux version would be nice (\*cough\* Blackmagic Design&#8230; you already release DaVinci for Linux), Wine 3 runs the software great. I actually tried this last year with an older version of WINE, but couldn&#8217;t get it to work. This time, all I had to do was open the MSI installer file with the WINE add/remove programs and bang! It worked! Awesome Stuff  
There was a bit of a struggle with the network configuration, though. Basically the switchers IP was something other than the default, and since WINE doesn&#8217;t support USB access for programs, we had to use another machine running macOS to configure the switchers static IP. Then it&#8217;s as simple as one end of the ethernet cable to the switcher, and the other to my laptop! 

Here&#8217;s a couple of extra pictures I took aswell: <figure class="wp-block-image">

![][2] </figure> 

That&#8217;s the full setup. You can see a small monitor to the left I used to view the schedule. The monitor above the laptop is plugged directly into the switcher to provide previews. On top of the swithcer is a BMD HDMI recorder, with a 1TB SSD in it. We pretty much filled that SSD up! 

<figure class="wp-block-image">

![][3] </figure> 

Close-up shot of the ATEM Switcher behind everything. 

<figure class="wp-block-image">

![][4] </figure> 

This little monitor was pretty useful, since we didn&#8217;t print a schedule for ourselves&#8230; 

<figure class="wp-block-image">

![][5] </figure> 

A look behind the setup. You can see the two SDI cables from the cameras. The audio mixer on the bottom was supposed to take in an audio feed we got from the audio guys (who were sitting in a proper booth behind us since they are actual pros&#8230;). For some reason the audio was just static, so we ended up using the cameras audio. 

<figure class="wp-block-image">

![][6] </figure> 

This is one of our cameras. The video from them is pretty nice quality. You can also see the audio booth behind there too. 

<figure class="wp-block-image">

![][7] </figure> 

Almost all of these seats were filled when the show started. The band performance was really good! 

<figure class="wp-block-image">

![][8] </figure> 

This is what the preview looks like when we have video input and graphics overlayed. 

<figure class="wp-block-image">

![][9] </figure> 

This is the SSD we use for recording. 

And that&#8217;s it! Doing this stuff is really fun, and even more so when we throw running the control software on GNU/Linux! Oh, and we had Pizza, so this was totally worth it.

 [1]: http://legacy.seshan.xyz/flow/files/WP_20181128_17_07_40_Rich.jpg
 [2]: http://legacy.seshan.xyz/flow/files/WP_20181128_17_22_29_Rich.jpg
 [3]: http://legacy.seshan.xyz/flow/files/WP_20181128_18_22_18_Rich.jpg
 [4]: http://legacy.seshan.xyz/flow/files/WP_20181128_18_22_48_Rich.jpg
 [5]: http://legacy.seshan.xyz/flow/files/WP_20181128_18_23_04_Rich.jpg
 [6]: http://legacy.seshan.xyz/flow/files/WP_20181128_18_23_11_Rich.jpg
 [7]: http://legacy.seshan.xyz/flow/files/WP_20181128_18_23_19_Rich.jpg
 [8]: http://legacy.seshan.xyz/flow/files/WP_20181128_18_47_59_Rich.jpg
 [9]: http://legacy.seshan.xyz/flow/files/WP_20181128_21_35_21_Rich.jpg