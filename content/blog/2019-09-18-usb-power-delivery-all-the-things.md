---
title: USB Power Delivery All The Things!
author: Seshan Ravikumar
type: post
date: 2019-09-18T19:31:49+00:00
url: /2019/09/18/usb-power-delivery-all-the-things/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
It&#8217;s 2019, and we&#8217;re seeing USB-C everywhere, and it&#8217;s awesome!

One of the things that really excites me is USB-PD, a technology that falls under the USB-C umbrella that allows devices to negotiate up to 100W of power.

Every since I got my new Macbook Pro, I&#8217;ve loved that fact that pretty much anything can charge my laptop. At the high end there is the official Apple 87 Watt charger, but I&#8217;ve even managed to trickle charge my Mac with a regular-old USB 2.0 wall adapter. It&#8217;s not actually that useful, but it does work!

I was recently helping someone fix their laptop (it was stuck in a Windows boot loop). One of the problems was he didn&#8217;t have his charger nearby, and without being able to actually boot Windows we had no idea if the laptop would last long enough (thankfully it did).

<hr class="wp-block-separator" />

And so herein lies the problem: the laptop had a USB-C port, but _didn&#8217;t_ support USB Power Delivery. And the worst part, there is no way to know this just by looking at the port. **And so begins the long list of USB-C problems&#8230;**

Even worse, there isn&#8217;t really a way to know the capabilities of a USB-C port visually at all. USB-C is a huge standard: USB-PD, ThunderBolt, Alternative Modes. Technically a USB-C Physical port can even just be a USB 2.0 host!

Even on a physical level there have been problems, back in 2016 there were reports of [cheap USB-C cables causing ports to explode][1]. Another example is the Nintendo Switch, which is _advertised_ as USB-C compliant but [actually deviates from the spec][2], both physically and electronically, causing many problems with third-party accessories.

Another problem is that USB-C isn&#8217;t that trivial to implement. As I&#8217;ve said, it&#8217;s a huge standard with lots of pieces. The USB-PD stack alone requires a fairly powerful CPU to be embedded in the USB Host controller. All of this makes a complete, standards compliant USB-C implementation fairly expensive, atleast in contrast to previous USB standards.

**Thankfully**, it doesn&#8217;t have to be this way and things are getting better! Compared to a few years ago, manufacturers are producing USB C ports with consistant support, and pretty much all the physical woes have been sorted out. Even better, USB 4 is around the corner. Gone will be the days of _USB C 2x(3.1) Gen 2 x 2^2 / 10 Power Delivery with ThunderBolt 3 and Alternative Modes._ One of the problems has been that Intel retained licenses rights over TunderBolt, meaning it&#8217;s implementation has been left out of a lot of devices (and as such it&#8217;s plethora of uses, such as eGPUs using PCIe over ThunderBolt).

So here is to a hopefully bright future where USB C will rule them all, and we can&#8217;t stop guessing at what the damn port actually can do.

 [1]: https://www.theverge.com/2016/2/4/10916264/usb-c-russian-roulette-power-cords
 [2]: http://the_switch_is_not_usbc_compliant_and_overdraws