---
title: Mobile, Web and Desktop Apps With One Codebase! (React Native/Expo)
author: Seshan Ravikumar
type: post
date: 2019-06-14T17:27:24+00:00
url: /2019/06/14/mobile-web-and-desktop-apps-with-one-codebase-react-native-expo/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
<div class="wp-block-image">
  <figure class="aligncenter"><img loading="lazy" width="1024" height="541" src="https://seshan.xyz/wp-content/uploads/2019/06/df9a79ae3b5de61a5b3199491a3fa6f9-1024x541.jpg" alt="" class="wp-image-140" srcset="https://seshan.xyz/wp-content/uploads/2019/06/df9a79ae3b5de61a5b3199491a3fa6f9-1024x541.jpg 1024w, https://seshan.xyz/wp-content/uploads/2019/06/df9a79ae3b5de61a5b3199491a3fa6f9-300x158.jpg 300w, https://seshan.xyz/wp-content/uploads/2019/06/df9a79ae3b5de61a5b3199491a3fa6f9-768x406.jpg 768w, https://seshan.xyz/wp-content/uploads/2019/06/df9a79ae3b5de61a5b3199491a3fa6f9.jpg 1920w" sizes="(max-width: 1024px) 100vw, 1024px" /></figure>
</div>

Nowadays if you want to launch a new software product or service, you&#8217;ll probably end up maintaining a few codebases for your App, website, etc. At worst, you&#8217;ll have separate versions for: 

<ul class="browser-default">
  <li>
    macOS
  </li>
  <li>
    Linux
  </li>
  <li>
    Windows
  </li>
  <li>
    iOS
  </li>
  <li>
    Android
  </li>
  <li>
    Windows
  </li>
  <li>
    Web
  </li>
</ul>

And possibly more. That&#8217;s a pretty bad case though, and you can usually reduce it to:

<ul class="browser-default">
  <li>
    Desktop (using Electron, Qt, etc)
  </li>
  <li>
    Mobile (using React Native, Xamarin, etc)
  </li>
  <li>
    Web (using React, Angular, etc)
  </li>
</ul>

This is pretty much what the current situation is like, but we can do better. What if&#8230;

<ul class="browser-default">
  <li>
    Mobile, Desktop, and Web (using React Native)
  </li>
</ul>

<div class="wp-block-image">
  <figure class="aligncenter is-resized"><img loading="lazy" src="https://seshan.xyz/wp-content/uploads/2019/06/1GkR93AAlILkmE_3QQf88Ug.png" alt="" class="wp-image-138" width="283" height="159" srcset="https://seshan.xyz/wp-content/uploads/2019/06/1GkR93AAlILkmE_3QQf88Ug.png 1000w, https://seshan.xyz/wp-content/uploads/2019/06/1GkR93AAlILkmE_3QQf88Ug-300x169.png 300w, https://seshan.xyz/wp-content/uploads/2019/06/1GkR93AAlILkmE_3QQf88Ug-768x432.png 768w" sizes="(max-width: 283px) 100vw, 283px" /></figure>
</div>

One codebase, all the platforms! React Native has come a long way, since the early days of being acceptable on iOS and kind of meh on Android. It&#8217;s since became a very competent and widely deployed solution for cross-platform mobile apps.

More recently, React Native was extended one more platform&#8230; the Web browser! Now, your initial thoughts may be: Why use React Native for web development when regular React exists? Well, the most obvious answer is using React Native for \*everything\* means you only need to develop a single codebase! Plus, react-native-web uses ReactDOM under the hood anyway.

<div class="wp-block-image">
  <figure class="aligncenter"><img loading="lazy" width="425" height="119" src="https://seshan.xyz/wp-content/uploads/2019/06/image-1.png" alt="" class="wp-image-105" srcset="https://seshan.xyz/wp-content/uploads/2019/06/image-1.png 425w, https://seshan.xyz/wp-content/uploads/2019/06/image-1-300x84.png 300w" sizes="(max-width: 425px) 100vw, 425px" /></figure>
</div>

Until recently, however, react-native-web was fairly annoying to use and cooperate with regular React Native. But now, thanks to the recent release of Expo v33, **there is a super elegant way to create truly universal apps that span across all devices!**

**Expo** basically bundles everything you need to create a React Native app into a streamlined process. It consists of a few parts, the Expo CLI to create/manage apps, Expo SDK which combines a bunch of native modules, and various Expo Devtools. Another major benefit of Expo is easy testing, especially on iOS: since all you need to do is download the Expo app from the store and launch your app!

As for desktop, you can embed the web version of your Expo in Electron. Awesome stuff!

[You can read more about the latest version of Expo, and trying out the new web support, from their blog post!][1]

 [1]: https://blog.expo.io/expo-sdk-v33-0-0-is-now-available-52d1c99dfe4c