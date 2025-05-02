<p align="center">
 <h1 align="center">LexiForge</h1>
</p>

[![License](https://img.shields.io/github/license/Broken-Deer/LexiForge.svg)](https://www.gnu.org/licenses/quick-guide-GPL-3.0.html)
![Contributors](https://img.shields.io/github/contributors/Broken-Deer/LexiForge?color=2green)
[![https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg](https://good-labs.github.io/greater-good-affirmation/assets/images/badge.svg)](https://good-labs.github.io/greater-good-affirmation)

A Rust implementation of a simple word book demo spec showcasing the use of [Vue] framework and [Tauri] framework.

This project can also be used as a real study tool.

## Installation

1. Download it from [release page](https://github.com/Broken-Deer/LexiForge/releases).
2. Run setup program (Windows), or install package through package manager (Linux)

## Usage

Choose the course and learn, the software can record your wrong words automatically, you can review them after study.

## License

The software is distributed under [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html) with additional terms.

### Additional terms under GPLv3 Section 7

1. When you distribute a modified version of the software, you must change the software name or the version number in a reasonable way in order to distinguish it from the original version. (Under [GPL-3.0, 7(c)](./LICENSE#L372-L374))

You need to find all the words related to the name of this program in the source code and replace them with the name of your own program

2. You must not remove the copyright declaration displayed in the software. (Under [GPL-3.0, 7(b)](./LICENSE#L368-L370))

3. If someone signs something of a contractual nature with the recipient and provides a commitment of liability, the licensor and author are not subject to this liability jointly and severally. (Under [GPL-3.0, 7(b)](./LICENSE#L382-L386))

## Manual build

Make sure you have completed [prep](https://v2.tauri.app/start/prerequisites) first.

Execute the following command to pull the code:

```bash
git clone https://github.com/conic-apps/launcher.git
cd launcher
```

Execute the following command to install the dependencies (in the project folder):

```bash
yarn install
```

Next, simply run the following command to complete the build:

```bash
yarn tauri build
```

If you want to contribute to this project, run `yarn tauri dev` to debug the application and see the details at [here](https://tauri.app/zh-cn/v2/guides/).
