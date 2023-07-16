# FileExplorer-Tauri
A small FileExplorer project for windows because I got bored :)

## About The Project
![image](https://dl.dropboxusercontent.com/scl/fi/dodwilga6wdvwhtvxjy5v/1.gif?rlkey=atftvlq9ttcrcjoffl55q5mox&dl=0)


This is a small project I made to learn more about the [Tauri](https://tauri.studio/en/) framework. It's a simple FileExplorer for windows. It's not perfect but it works. I will probably add more features in the future.

## Getting Started
### Prerequisites
* [Node.js](https://nodejs.org/en/)
* [Tauri](https://tauri.app/v1/guides/)
* [Rust](https://www.rust-lang.org/tools/install)

### Installation
1. Clone the repo
   ```sh
   git clone https://github.com/k3rn3lpanicc/FileExplorer-Tauri.git
    ```
2. Install NPM packages
    ```sh
    npm install
    ```
3. Build the project
    ```sh
    npm run tauri build
    ```
4. Run the project
    ```sh
    npm run tauri dev
    ```
## Usage
* Click on a folder to open it
* Double Click on a file to open it with the default program
* Right Click in a folder to open a context menu
* Click on the "Back" button to go back to the previous folder
* Search for a file or folder by typing in the search bar, you can use recursive search by checking the "Recursive" checkbox

## Search Benchmark
This isnt a valid and accurate benchmark but it gives you an idea of how fast the search is. I searched for a file called "fixluminance.htm" in my C drive, and in windows explorer it took 4 minutes and 39 seconds to find the file, but in my FileExplorer it took only 40 seconds to find the file. I know it's not a fair comparison because windows explorer includes more stuff in the search but it's still a huge difference. I will try to improve the search algorithm in the future using BTrees, Hashmaps and chaching.

![image](https://github.com/k3rn3lpanicc/FileExplorer-Tauri/assets/20683538/46e96732-f426-40ab-9e9e-fa813363853c)

![image](https://github.com/k3rn3lpanicc/FileExplorer-Tauri/assets/20683538/46155e2d-0751-45f2-8bc0-5a5249279892)



## License
Distributed under the MIT License. See `LICENSE` for more information.

## Contact
* Discord: k3rn3lpanicc#0001
* Email: matin.ghiasvand1381@gmail.com
* Telegram: [@k3rn3lpanic](https://t.me/k3rn3lpanic)

