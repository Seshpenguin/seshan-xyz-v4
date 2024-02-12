---
title: My thoughts after using macOS
author: Seshan Ravikumar
type: post
date: 2019-03-25T22:22:44+00:00
url: /2019/03/25/my-thoughts-after-using-macos/
categories:
  - Uncategorized

---
Wow, two blog posts in a row??? It seems these posts come in bursts.  
I&#8217;ve been doing somethings that are probably worthy of a blog post, so here is one. <figure class="wp-block-image">

![][1] </figure> 

Mac OS X! Wait, no.. OS X! Hold on&#8230; macOS! Naming jokes aside (maybe we should call it Darwin/Cocoa&#8230;), I&#8217;ve been using macOS the past few days for curiosities sake, and it&#8217;s actually quite nice. Now a bit of note here,  
I still use GNU/Linux and free software all the time, and this (I hope) is temporary. Nevertheless using macOS is pretty fun, probably because I get to feel like I&#8217;m using something new and different again.  
  
Ok so, overall macOS is really enjoyable to use. It&#8217;s POSIXiness and UNIXiness makes me feel right at home. I even installed the GNU coreutils and Bash using brew (more on that later) to make everything nice and cozy.  
Definelty one of the selling points of macOS is how polished it is. Every time I go to do something new on it, I am almost always surprised with attention to small details that really make the user experience pleasant.  
Random example when making this post, I wanted to delete the serial number from the screenshot, and so I opened it using the default macOS image preview tool. When I dragged over the part of the image, and pressed delete, it didn&#8217;t just delete the section, but also filled it in with the surrounding colour.  
  
Let&#8217;s take a break from the positives and talk about some negatives for a second. The first one is pretty obvious, macOS is proprietary software. Technically the very base (Darwin, ie. Kernel and BSD userspace) is free/libre, but the majority is still closed. Another thing actually contrasts the polish point. A lot of the polish of macOS seems to be in areas that most regular users would encounter them in normal usage. However, as you start to get futher into the system, as I do as a &#8220;power-user&#8221; and developer, you start to notice some things can be annoying.  
For example, running apps that aren&#8217;t approved by Apple typically needs you to manually approve them to run by going to the settings. Also, Apple also tends to try and hide parts of the low level OS, which I usually like to touch. Finally, if you like/have to use legacy software, Apple has never made that easy. For a course in school they use a lot of documents using Pages &#8217;09. I was excited, since I was using macOS, that I could get that version of iWork and use my laptop instead of the school&#8217;s iMacs. Sadly, however, iWork &#8217;09 didn&#8217;t work that well in macOS Mojave. It was really slow, and now Pages won&#8217;t even launch. And don&#8217;t try to use an old version of macOS either, you won&#8217;t get much support past maybe the last two or three versions.  
  
Back to some positives, the TOUCHPAD! I&#8217;ve hated using the touchpad my entire life, being a previous Windows user and current Linux user. What I never realized is just how pleasant the touchpad experience is on macOS. Gestures are really fluid and convenient. I particularly enjoy macOS&#8217;s maximize behavior, where applications go fullscreen into their own workspace of sorts. When they do that, I can 3-finger swipe between them. At first I thought this might be annoying, but has made doing work with only one monitor a breeze. It&#8217;s quite nice. I think this is something the Linux community need to work on, is the touchpad experience. Small things like cursor acceleration feels very refined on macOS.  
  
Homebrew, probably one of the most popular package managers for macOS, has made macOS usable for me. Without something that can give me the GNU/Linux packages I know and love (and use on the daily), macOS would have been DoA for me. It would be nice if Apple gave macOS it&#8217;s own supported package manager, but it doesn&#8217;t surprise me that they didn&#8217;t. Another cool package manager for macOS I tried was MacPorts. Similar to the BSD&#8217;s ports system, MacPorts compiles it&#8217;s software from scratch. One of the things I liked about MacPorts was the larger software selection, including a lot of Linux GUI software. I managed to get a full XFCE4 desktop running beside Apple&#8217;s Aqua DE! macOS has a program call XQuartz (which used to be officially supported) that lets you run X11 applications. There is also Fink, which apparently uses apt and dpkg (with it&#8217;s own packages of course), but I have yet to try. It&#8217;s also worth noting you can&#8217;t run multiple packages managers at the same time, which makes sense.  
  
So yea, I could probably write more, but I&#8217;m too lazy&#8230; So, TL;DR, macOS is nice, very polished and I wouldn&#8217;t be too mad if I was forced to use it. It&#8217;s definitely miles ahead of Windows. I think I&#8217;ll stick to Ubuntu Linux, though.

 [1]: http://legacy.seshan.xyz/flow/files/mac-screenshot.png