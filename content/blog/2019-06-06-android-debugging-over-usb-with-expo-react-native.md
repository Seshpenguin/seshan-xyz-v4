---
title: Android Debugging over USB with Expo (React Native)
author: Seshan Ravikumar
type: post
date: 2019-06-06T13:17:34+00:00
url: /2019/06/06/android-debugging-over-usb-with-expo-react-native/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
<div class="wp-block-image">
  <figure class="aligncenter"><img loading="lazy" width="425" height="119" src="https://seshan.xyz/wp-content/uploads/2019/06/image.png" alt="" class="wp-image-104" srcset="https://seshan.xyz/wp-content/uploads/2019/06/image.png 425w, https://seshan.xyz/wp-content/uploads/2019/06/image-300x84.png 300w" sizes="(max-width: 425px) 100vw, 425px" /><figcaption>Expo is a pretty awesome React Native development framework.</figcaption></figure>
</div>

I&#8217;ve been working with React Native a lot recently. And so while the straight React Native life is great, compiling your apps with native dependencies (I&#8217;M LOOKING AT YOU IOS) can get&#8230; annoying. Thankfully Expo makes that easy.

By default though, Expo wants you to debug your apps over the network. This is great, but sometimes your development machine and phone are not on the same network. They do provide a solution for this, the tunnel, but it&#8217;s kind of slow since all your debugging traffic has to route through a cloud server.

**There is a solution, however! Do it over USB (like the good old days with react-native run-android).** 

When you launch Expo, it should open the devtools in your browser. When it&#8217;s loaded, in the bottom right corner, select **&#8220;Local&#8221;.**<figure class="wp-block-image">

<img loading="lazy" width="1024" height="538" src="https://seshan.xyz/wp-content/uploads/2019/06/Screenshot_2019-06-06-7-ROADA-Hours-on-Expo-Developer-Tools-1024x538.png" alt="" class="wp-image-106" srcset="https://seshan.xyz/wp-content/uploads/2019/06/Screenshot_2019-06-06-7-ROADA-Hours-on-Expo-Developer-Tools-1024x538.png 1024w, https://seshan.xyz/wp-content/uploads/2019/06/Screenshot_2019-06-06-7-ROADA-Hours-on-Expo-Developer-Tools-300x158.png 300w, https://seshan.xyz/wp-content/uploads/2019/06/Screenshot_2019-06-06-7-ROADA-Hours-on-Expo-Developer-Tools-768x404.png 768w, https://seshan.xyz/wp-content/uploads/2019/06/Screenshot_2019-06-06-7-ROADA-Hours-on-Expo-Developer-Tools.png 1853w" sizes="(max-width: 1024px) 100vw, 1024px" /> </figure> 

After that, in the terminal where you started Expo, you should see a message like:

<blockquote class="wp-block-quote">
  <p>
    Successfully ran <code>adb reverse</code>. Localhost URLs should work on the connected Android device.
  </p>
</blockquote>

That means Expo was able to see your Android device! If you don&#8217;t see that, make sure your device is plugged in and [USB Debugging][1] is enabled.

**Now all you need to do is press &#8220;a&#8221; in the terminal. With any luck you should see your app launch on your device!** No networking here!

 [1]: https://developer.android.com/studio/debug/dev-options