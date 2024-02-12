---
title: macOS Catalina now supports VirtIO (Linux Paravirtualized KVM Drivers)!
author: Seshan Ravikumar
type: post
date: 2019-09-20T20:49:30+00:00
url: /2019/09/20/macos-catalina-now-supports-virtio-linux-paravirtualized-kvm-drivers/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
Holy crap guys! After watching [LTT&#8217;s new video][1] about a dual Windows-macOS system using KVM on Linux, they talked about how the macOS Catalina beta has VirtIO support. 

**WHAT? REALLY?**

Yes! It&#8217;s true! [From what I have read][2], macOS Catalina currently has support for display adapters (stdvga and cirrus), virtio-blk (for drive passthrough) and virtio-9p (for drive sharing).

Obviously this is exciting for everyone who wants to run macOS on QEMU (through KVM on Linux or hypervisor.framework on macOS), but the speculation is Apple is adding VirtIO support for the enterprise crowd, since their upcoming Mac Pro could be a nice Mac virtualisation platform (recall that Apple advertised the Mac Pro as support rack mount configurations).

So there you go: hackintosh-ers, tinkerers, and enterprise folk alike, rejoice! macOS is becoming a friendlier guest OS.

 [1]: https://www.youtube.com/watch?v=EozeSDeV3Vo
 [2]: https://passthroughpo.st/mac-os-adds-early-support-for-virtio-qemu/