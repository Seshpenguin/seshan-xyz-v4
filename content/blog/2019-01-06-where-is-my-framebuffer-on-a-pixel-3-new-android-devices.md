---
title: Where is my Framebuffer?? (On a Pixel 3 / New Android devices)
author: Seshan Ravikumar
type: post
date: 2019-01-06T21:29:38+00:00
url: /2019/01/06/where-is-my-framebuffer-on-a-pixel-3-new-android-devices/
categories:
  - Uncategorized

---
Ahh Android, everyone&#8217;s second favorite distribution of Linux (behind GNU/Linux, of course). I have a Pixel 3 XL, rooted et al. But recently I&#8217;ve wanted access to a full GNU/Linux environment right in my pocket. Now, setting that up isn&#8217;t particularly challenging. Android itself runs on the Linux kernel, and the Toybox userland (a fork of busybox) has everything needed for a nice chroot. Within that chroot (running, say Debian or Ubuntu), you can run pretty much anything, standard programs, ssh server, and even a X server!  
  
Ah&#8230; X11. There really 2 main ways people get graphics working on their chroots on Android. Way #1 is to run X11 in the chroot with VNC, and then VNC into the chroot. The other way is to get an X11 server as an app. Both ways are fine and dandy, but there is a third, and in my opinion, more interesting way. That is to run X11 in the chroot, but have it draw directly to the phones framebuffer. Remember, this is Linux, so we can do that! Just stop &#8220;SurfaceFlinger&#8221; (Android&#8217;s Display server), and we can take over! So naturally, I went and tried to setup my Debian chroot with the framebuffer, but I ran into an interesting problem&#8230;  
  
**Where is my framebuffer???** On Android, I should expect to see a &#8220;/dev/graphics/fb0&#8221;, but it wasn&#8217;t there. Now, this is where about an hour of searching online and poking around led me to. As it turns out, I don&#8217;t have a framebuffer device! Why? Well, the Google Pixel line of phone turns out to the the first Android devices to use KMS/DRM.  
  
What is KMS/DRM? No, it&#8217;s not some windows activator and digital rights management. KMS stands for Kernel Modesetting and DRM is Direct Rendering Manager. Together they make up mainline Linux&#8217;s graphics stack. I won&#8217;t get too much into detail here, but basically it&#8217;s the modern way to do graphics in Linux. In the past, it was up to the display server to handle much of the graphical things when it comes to talking with the hardware. In GNU this would be the task of X11, and in Android it was the task of Atomic Display Framework. Both needed a framebuffer device to talk to the graphics card through the kernel. Nowadays though, things have changed, and the Linux kernel now uses KMS and DRM, which is a much nicer way of interacting with graphics hardware. It&#8217;s actually really nice that Android has adopted the mainline graphics stack, there is less and less out-of-tree code in the Android Linux kernel!  
  
But anyway, sadly my plans for using the framebuffer for my chroot may be a bit harder now. It is probably possible to use KMS/DRM from the chroot, but this is something entirely new, and I don&#8217;t think anyone has done this before. I will keep you guys updated if I get somewhere with this!  
  
Btw random website update: everything broke when I published this post (id 10). I have no idea why, but I changed some things and it seems fine now&#8230;