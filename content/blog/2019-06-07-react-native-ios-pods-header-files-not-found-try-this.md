---
title: 'React Native iOS Pods: Header Files not found? Try this!'
author: Seshan Ravikumar
type: post
date: 2019-06-07T13:00:09+00:00
url: /2019/06/07/react-native-ios-pods-header-files-not-found-try-this/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
As anyone who has worked with iOS development will know&#8230; IT&#8217;S REALLY ANNOYING. One problem I faced with React Native and Pods (Xcode dependency manager) is that you actually need to add a bunch of RN specific stuff to your Podfile for it to work properly. 

**If you are having compile issues with your Pods, check this link and make sure your Podfile has what it needs!**

<https://facebook.github.io/react-native/docs/integration-with-existing-apps>

<pre class="wp-block-code"><code># The target name is most likely the name of your project.
target 'NumberTileGame' do

  # Your 'node_modules' directory is probably in the root of your project,
  # but if not, adjust the `:path` accordingly
  pod 'React', :path => '../node_modules/react-native', :subspecs => [
    'Core',
    'CxxBridge', # Include this for RN >= 0.47
    'DevSupport', # Include this to enable In-App Devmenu if RN >= 0.43
    'RCTText',
    'RCTNetwork',
    'RCTWebSocket', # Needed for debugging
    'RCTAnimation', # Needed for FlatList and animations running on native UI thread
    # Add any other subspecs you want to use in your project
  ]
  # Explicitly include Yoga if you are using RN >= 0.42.0
  pod 'yoga', :path => '../node_modules/react-native/ReactCommon/yoga'

  # Third party deps podspec link
  pod 'DoubleConversion', :podspec => '../node_modules/react-native/third-party-podspecs/DoubleConversion.podspec'
  pod 'glog', :podspec => '../node_modules/react-native/third-party-podspecs/glog.podspec'
  pod 'Folly', :podspec => '../node_modules/react-native/third-party-podspecs/Folly.podspec'

end
</code></pre>