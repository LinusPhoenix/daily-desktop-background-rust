# Daily Desktop Background (Rust)

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

A small Rust application that changes your desktop wallpaper to a random image from [Unsplash](https://unsplash.com/), using their free API to get a random photo found when searching for "desktop background". Put a shortcut to the executable in your Autostart folder and enjoy a different desktop background everytime you boot up your PC!  
Inspiration for this project came from [40 Side Project Ideas for Software Engineers](https://www.codementor.io/@npostolovski/40-side-project-ideas-for-software-engineers-g8xckyxef) (idea #30).  
This application should work any platform supported by [wallpaper.rs](https://github.com/reujab/wallpaper.rs/).

## The Unsplash API

Unsplash requires you to authenticate yourself when using their API, which means you need to register your application. In their API guidelines (as of June 2020), it says that applications "cannot replicate the core user experience of Unsplash (unofficial clients, **wallpaper applications**, etc.)" (emphasis mine). In an email I was told that this does apply to this particular project, but I could use it for personal use anyway (meaning "Demo" app classification and a limit of 50 requests/hour).  
This unfortunately means that you will need to get your own access key, which you can do so [here](https://unsplash.com/developers) (it's free), and build this project yourself if you want to use it.

## Building the project

First, open `src/.access-key` and replace the placeholder text there with your own Unsplash API acess key. Make sure that nothing else is in that file.
Then you can use `cargo build`, `cargo build --release`, `cargo run` as normal.

## Was Rust **really** the right tool for the job?

I wanted to try Rust after reading [The Rust Programming Language](https://doc.rust-lang.org/book/) and while this project is small and uses barely any features, it was fun to explore the Rust ecosystem and just basic syntax. Before discovering wallpaper.rs, I was doing file operations and registry editing myself, but ultimately despaired when dealing with Windows permission system.
It's a lot better than the .NET Core version, with a slim 16MB executable that performs noticeably faster.

I also wrote a (Windows-only) version of this in .NET Core: [DailyDesktopBackground](https://github.com/LinusPhoenix/daily-desktop-background)
