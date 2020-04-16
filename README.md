# Amethyst-Iced

An Amethyst addon to create your game UI using Iced. 

## Features

* Plug-and-play, just add a Render Plugin and a Bundle to your GameData, then get started with Iced
* Most (but not all) components of `iced_native` are usable
* Correctly handles resizing. Responsive Game UIs ftw !

## Motivation 

Writing UIs is both complex and important for video games. As [Amethyst](https://github.com/amethyst/amethyst) strives to be a powerful and feature-complete game engine, and [Iced](https://github.com/hecrj/iced) an easy-to-use and type-safe GUI library, I thought it would be a good idea to glue them together, and provide an alternative to Amethyst's built-in UI System. 

## Demo 

![Pane-Grid](screenshots/panegrid.gif)
![Progress Bar](screenshots/progressbar.png)

## Usage 

See the examples. Sorry for the lack of comments.

## Todo-list

* Improve the global code quality of the codebase and of the examples. This crate is poorly documented, and was written merely as a proof of concept. 
* Review/Audit the IcedUI Rendy plugin as I am confident I have done a horrible job at "Rendy best practices". 
* Fix radio buttons so they don't look like Checkboxes. Broadly speaking, design a way of using Lyon to render shapes, possibly getting inspiration from amethyst_lyon.  
* Support custom font loading for Text widgets
* Support Iced's debugger
* Support Images and Nine-Patches using custom styling for a variety of widgets where it would be applicable : 
    * Buttons, 
    * Checkboxes, 
    * Sliders, 
    * Radio, 
    * Progress Bar 
* Write an example for actual game integration 
* Fix winit support which is in a poor state right now by using iced_winit once the Amethyst engine has done the transition to winit 0.20  
* Support for Futures (possibly iced_futures) and implementation of the Application trait like in the Iced base crate
* Implement Scrollable widget
* Implement TextInput widget
* Implement amethyst profiling
* Port more examples from iced
* Setup CI
* Add badges to this repository. This is a serious project, after all.

Help would be appreciated ! :)
