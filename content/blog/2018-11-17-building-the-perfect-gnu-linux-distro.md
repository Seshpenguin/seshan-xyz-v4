---
title: 'Building the perfect GNU/Linux Distro*'
author: Seshan Ravikumar
type: post
date: 2018-11-17T19:11:10+00:00
url: /2018/11/17/building-the-perfect-gnu-linux-distro/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
<blockquote class="wp-block-quote">
  <p>
    IGNORE THIS POST!!!
  </p>
</blockquote>

**Hey! How&#8217;s it going**  
In my ever lasting quest of being way too bored despite having tons of work to do&#8230; I present to you my &#8220;plan&#8221; for what I would consider a &#8220;perfect Linux distro&#8221;. 

Ok so let&#8217;s get right into it: Basically there are a few fundamental concepts here. First is that the OS itself should be fairly stable. By that I mean the basic userspace shouldn&#8217;t be rolling (sorry Arch fans, but you might enjoy a feature i&#8217;ll talk about later).  
As a base, I was thinking maybe CentOS? I&#8217;ve used Fedora a lot and have noticed it behaves really nicely on all sorts of hardware and software configuration.  
CentOS is has a much slower release cycle, but we can probably backport changes as need be. The slightly older software is ok because&#8230;  
Snaps!  
The next big part of the OS would be that apps are handled independent of the base OS. This means smaller and less frequent system updates, but still up to date software.

So, a solid base and snaps for apps. Btw, I am currently experimenting (to various degrees of success) with compiling the CentOS SRPMS myself. While it&#8217;s not entirely necessary, it would be quite nice if I can compile the system.  
The choice of Desktop Environment is probably the next big choice, since it&#8217;s the part that people actual touch and use. I&#8217;ve used a number of DEs before, including GNOME, MATE, KDE, etc. They are all great choices, but i&#8217;m thinking something a bit different.  
The Budgie DE from the Solus Project is actually quite nice, and I&#8217;ve used it extensively too. It&#8217;s basically GNOME, but makes sense&#8230; It has a much lower RAM and CPU overhead, and is nice and clean looking. It&#8217;s built with a lot of the same GNOME technologies,  
so it keeps a lot of the eye candy and overall feel of the GTK world.  
The big issue with Budgie right now is that I would need to compile it myself to target CentOS. This is going to be pretty interesting though, since it&#8217;ll mark the point where we start diverging from CentOS, since we&#8217;ll need to start updating some system libraries to meet  
Budgies dependency requirements&#8230; Fun!

Couple of other random ideas I wanted to throw out there too. Sometimes you really want to test out a new piece of software, or the software you want isn&#8217;t on the repos or snap store. Well..

Arch Compatibility Layer! Basically a nicely integrated Arch Linux chroot so you can install all sorts of bleeding edge software and software from the AUR. 

I&#8217;ve got some other ideas but yeaaa. Heck maybe throw out this one and make a distro with Ubuntu and UBPort&#8217;s Unity 8?? 

***Edit: Please diregard this post my views have changed&#8230; 😛 Coming soon debian distro- wait no coming soon arch based non systemd- uh yea idk**