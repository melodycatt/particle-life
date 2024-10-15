# Usage
To run the sim, you will need rust installed. 
For Linux and MacOS, install rustup by running the command below in your console and select default install. (On MacOS this installation or trying to run the sim may prompt you to install XCode tools)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --help
```
For windows, download [rustup-init.exe](https://www.rust-lang.org/tools/install)

Once you have installed rustup, clone this repository with git or download the zip from the green code button at the top of this page and extract it. Navigate to the particle-life folder in your terminal, or open a new terminal at the folder. Finally, run this command and you're ready:
```
cargo run -r --bin main
```