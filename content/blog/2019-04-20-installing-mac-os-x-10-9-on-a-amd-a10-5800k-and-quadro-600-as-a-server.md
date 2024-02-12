---
title: Installing Mac OS X 10.9 on a AMD A10-5800k and Quadro 600 as a server.
author: Seshan Ravikumar
type: post
date: 2019-04-20T22:23:55+00:00
url: /2019/04/20/installing-mac-os-x-10-9-on-a-amd-a10-5800k-and-quadro-600-as-a-server/
categories:
  - Uncategorized

---
Hey everybody! It&#8217;s that time again: let&#8217;s do something entirely random and probably useless! <figure class="wp-block-image">

![][1] </figure> 

Check this thing out! This is my old computer, which to be honest I don&#8217;t really remember when I built it, but it was probably back around 2014 if I had to guess. Specs wise it&#8217;s not too bad: AMD A10-5800k with 8GB of RAM. The HDD is a 1TB WD Green and was pulled out of a USB HDD. As for graphics, this computer spent most of it&#8217;s life being used with the integrated Radeon graphics (which was good \*enough\*).  
You may have noticed the GPU in the picture though, just a few weeks ago EspiDev decided to give me a Quadro 600, so I decided to throw that in there too.  
  
Ok&#8230; now for the fun stuff, let&#8217;s install Mac OS X on it! Why? Uh&#8230; who knows. ¯\_(ツ)_/¯  
Ok so the first Hackintosh distro I tried was Niresh&#8217;s OS X 10.7 Lion. Two reasons, one because I thought Lion would suit this hardware well (I was wrong), and it was the last release that would fit on a single layer DVD (< 4.7G).  
I also want to take this moment to point out that this is probably one of the more sketchy computers to put OS X on, since it has an AMD CPU. If you don&#8217;t know: the stock Darwin kernel in OS X basically doesn&#8217;t work at all, so these Hackintosh distros ship custom compiled kernels with AMD support. These kernels are of dubious quality and \*probably\* shouldn&#8217;t be trusted. Anyway, on with the install: <figure class="wp-block-image">

![][2] </figure> 

I just followed the instructions to load the AMD kernel and boot, and to my surprise it actually booted!  
I just partitioned the disk, selected the drivers I thought I would need (GPU, Network, etc), and it started installing. This was going pretty smooth&#8230; <figure class="wp-block-image">

![][3] </figure> 

What!? It just rebooted into the OOBE just like that? Actually no.. for some reason the bootloader refused to work upon reboot. A quick Google search revealed that this old version of Chameleon doesn&#8217;t like the larger sector sizes of my HDD. Not to worry though, I just used the DVD&#8217;s boot loader to load OS X. <figure class="wp-block-image">

![][4] </figure> 

And we are in! Woo! Time to celebrate&#8230; not so fast. As it turns out Lion was just a little too old to support this hardware properly. The Nvidia GPU was recognized, but didn&#8217;t seem to actually be accelerating anything. Not much of an issue since I intend to use this computer as a server. The problem though, was the network card. I tried and tried to load all sorts of kexts to no avail.  
  
At this point I realized Lion probably wouldn&#8217;t work. At this point I tried High Sierra, which I though might work. Sadly though, the Hackintosh builds of High Sierra target AMD CPUs too new compared to this A10 (they probably work on Ryzen generation). I didn&#8217;t want to give up, though.  
I decided to give OS X 10.9 Mavericks a shot. Lion worked (it atleast booted), High Sierra didn&#8217;t, so I hoped Mavericks would be the perfect middle ground. Here is the problem though: It refused to boot from USB. Remember how I said Lion was the last OS X version to fit on a Single Layer DVD? With High Sierra I had to boot from a USB. This worked, but for some reason Mavericks just didn&#8217;t have any of it. I tried using Disk Utility, dd (with dmg2img), the built in Niresh flashing tool. Nothing seemed to make the USB boot work. <figure class="wp-block-image">

![][5] </figure> 

Verbatim to the rescue! I was in Best Buy a few days later (unrelated to this adventure) and wonder if I could find some Dual Layer DVDs while I was there. And as it turns out, in literally the corner of the store was a couple of packages of (fairly pricey) Dual Layer DVDs. Awesome! I burned 10.9 onto the DVDs and hoped for the best. <figure class="wp-block-image">

![][6] </figure> 

And it worked! Everything from the GPU, to sound, to (most importantly) the network card! I was right, OS X 10.9 was the sweet spot. 

Now you may be wondering what I actually plan on using this for. I will say that you may cringe, so viewer discretion is advised.  
Since this is a 1TB HDD I decided this would work as a nice storage server. So, I used OS X&#8217;s built in SMB server, and that worked no problem. The only thing though is I wanted a way to access to access my files remotely (and one does not simply expose an outdated SMB server to the network). I&#8217;ve used NextCloud before but I don&#8217;t believe it works (not easily, anyway) natively on Mac OS. So I installed VirtualBox and used Turnkey Linux&#8217;s NextCloud image (hey, I was pretty lazy after all this). Surprisingly this works and pretty darn well. NextCloud&#8217;s External Storage plugin plays nice with OS X&#8217;s SMB server. A little while later I setup some other services on OS X, like Transmission so I can seed some Linux ISO torrents and whatnot (with Combustion as the WebUI, it&#8217;s pretty slick).  
Please feel free to vent your anger.  
  
So there you go, quite the adventure and now I have a pretty sketchy server. Who knows how long it will last. I also want to point out that this took longer than it may have seemed, it spanned over a few days and a lot of trial and error.  
Anyway, se you guys next time!

 [1]: http://legacy.seshan.xyz/flow/files/IMG_20190327_131143.jpg
 [2]: http://legacy.seshan.xyz/flow/files/IMG_20190327_162611.jpg
 [3]: http://legacy.seshan.xyz/flow/files/IMG_20190327_165610.jpg
 [4]: http://legacy.seshan.xyz/flow/files/IMG_20190327_165846.jpg
 [5]: http://legacy.seshan.xyz/flow/files/IMG_20190329_214555.jpg
 [6]: http://legacy.seshan.xyz/flow/files/IMG_20190330_095505.jpg