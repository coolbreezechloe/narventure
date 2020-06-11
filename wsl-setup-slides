SETUP
=====

Windows
-------

1. Turn on windows subsytem for linux

    Method1
    *******

    * Press windows key
    * Type "Power Shell"
    * Right click on the icon and select run as administrator
    * Type
        `Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux`
    * You may need to restart the computer

    Method2
    *******

    * press windows key
    * type "windows features"
    * press enter
    * find "Windows Subsystem for Linux" in the list and select it
    * Press OK
    * May need to restart

1. After reset install debian
   * open microsoft store
   * search for debian and "get" and "install" it
   * click launch after it is installed. At this point you can close the
     microsoft store
   * go through the install process which will appear in a text window and
     starts with asking for your new UNIX username


1. Install Rust
   * run the command from `https://rustup.rs` in the terminal
   * select "proceed with installation"
   * exit and restart the debian shell

1. Confirm installation with command `rustup toolchain list`
1. Install Build Tools for WSL
    * sudo apt install gcc


ACCESSING DOCS
==============

1. sudo apt-get install python3
2. cd ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html
    * if directory does not exist try `rustup doc --book` and notice the location
      of the file it tried (unsuccessfully) to open
    * make a link to this folder so you don't have to remember it next time with this:
      1. `ln -s <big long folder> ~/rustdocs`
      1. now instead of typing `cd <big long folder>` all the time, just type `cd ~/rustdocs`
3. python3 -m http.server
4. open web browser to http://localhost:8000/



TERMS
=====

rustaceans: rust-ah-kins, describes people who program in Rust

Constants
---------

* Scope can be global
* Use `const` keyword, it is not the same as an immutable variable
* Naming Conventions. Typically all upper case with underscores for spaces
* Not mutable
* Must be annotated
* Must be set to a constant expression, can not be the result of a function

Shadowing
---------

* A new variable with same name as a perviously defined variable
* Rustaceans say the first variable is shadowed by the second
* <example>
* can change the type
* with let we can perform a few changes on a variable without having it
  by mutable at the end


