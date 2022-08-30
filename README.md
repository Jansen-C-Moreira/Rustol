<h1 align=center><code>Rustol</code></h1>

<p align="center"><img src="https://jansencmoreira.tech/wp-content/uploads/2022/08/rustolImage.png"></p>

## Introduction


`Rustol` is a static analyzer for solidity contracts based on [c4udit](https://github.com/byterocket/c4udit).
<p align="center"><img src="http://jansencmoreira.tech/wp-content/uploads/2022/08/drawio.png"</p>

It is capable of finding low-risk issues and gas optimization(medium and highs in the future) by using regular expressions specified by the user. What we call `rules`.
  
Note that in `Rustol`, at the actual version of the tool, the user has to create two folders to be able to run this project if opted by the second installation method:

* `rules/` - Where the rules are located
* `test_files/` - Where the solidity files are located
obs.: Better explanation [here](https://jansencmoreira.tech/?project=rustolprototype).


## Installation

There are two ways to use `Rustol`:
  * By cloning this repository and running `cargo build`. But, you must have rust installed.
  * Or; Download the rustol.exe above from the repository and create the required folders.
 
  By opting for the first option, if you run it in a terminal with color support, the information will be printed with colors.
  But, if you don't want to install rust, you can just download the executable file. The only problem, it's that the result will come via file, `result.md` without colors (I'm working on that).
  
 ## Features
  
  `Rustol` allows the user to create rules by using the CLI(Command Line Interface) or by making a file with the id(1st line), the description(2st line), and the regex(3st line):
  ```
1
[L-01][LOW-RISK] - Pragma Floating (https://swcregistry.io/docs/SWC-103)
pragma solidity  (\^|>)
  ```
  
  To create a new rule, using the CLI, you have to execute the script/program and type `s` or `S`:
  ```
███████████                       █████             ████  
░░███░░░░░███                     ░░███             ░░███ 
 ░███    ░███  █████ ████  █████  ███████    ██████  ░███ 
 ░██████████  ░░███ ░███  ███░░  ░░░███░    ███░░███ ░███ 
 ░███░░░░░███  ░███ ░███ ░░█████   ░███    ░███ ░███ ░███ 
 ░███    ░███  ░███ ░███  ░░░░███  ░███ ███░███ ░███ ░███ 
 █████   █████ ░░████████ ██████   ░░█████ ░░██████  █████
░░░░░   ░░░░░   ░░░░░░░░ ░░░░░░     ░░░░░   ░░░░░░  ░░░░░  
                   Hello, Welcome to Rustol 😀

---> Made by Jansen Cersosimo Moreira
site: https://jansencmoreira.tech
email: jansenc@jansencmoreira.tech


---------Starting----------

----Rule-Creation----

Want to add a new rule?(s/N):
s
id:
5
Description:
rule x
Regex:
regex(///)
  ```
  
  ## RoadMap
I just learned rust, so I have plans to improve this tool. But, first, I will improve what I already got: 
  
Early goals:
* Give more freedom to the user. Let him decide the name of the rules folder and test files folder.
* Fixing the colors and the Result.md
* Improving the code. Less code, better performance.
  
After improving some basic stuff, I will go directly to high-level features:
  
Mid goals:
* Add new ways to find bugs. (Regex is basic)
* Generate automatic POCs for certain vulns and bugs
* Add this tool as a plugin in brownie and other solidity frameworks. In this way, you can execute it at the time you finished coding your solidity file.
  
If I have some time left:
  
End goas:
* Add AI to find and tag vulns
* ...

 ## Credits
  
  `Rustol` was developed by [Jansen Cersosimo Moreira](https://jansencmoreira.tech) in three to four hours for practicality and learning purposes.
  Thus, if you have any ideas, comments, feedback, or doubts, I would be pleased to know about it, so please contact me:
  
  LinkedIn: https://www.linkedin.com/in/jansen-moreira/?locale=en_US
  
  Email: jansencm@jansencmoreira.tech
  
