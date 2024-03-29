# Git's smart change directory (gcd)

It is a CLI tool that performs "change directories" in git repository space very short and smart.
If you have a unique directory name under the control of git, you can skip the layered PATH and move by just the directory name.

## About
- Git cannot add a completely empty directory. Many people who want to maintain an empty directory in Git have created a convention of putting a file called ".gitkeep" in that directory.

- The program uses the .gitkeep file to do the following :  
  - After adding the file to an empty DIR, use the ".gitkeep" file without deleting it.
  - It will create a list of PATH for the directory where the ".gitkeep" file is located.
  - It then compares that list with the name of the destination directory and changes it directly to the full path with a forward match.
  - Do a Change Directory using this. (If you "cd" many times, use a unique DIR name.)
  - Note: Since it refers to .git/logs/HEAD, "gcd" does not work in a repository with 0 commits.

## Example 
1. For example, there is a sample directory at the top of the Git repository. In addition, there are sample/hier/route1/hierA/hierB/hierC and sample/hier/route2/levelA/ levelB/LevelC in the working DIR.
1. This is an example of moving to "hierC" first and then to "LevelC".
- `cd` : (Standard change directory)
    ```bash
    $ cd sample/hier/route1/hierA/hierB/hierC    
    $ cd ../../../../route2/levelA/levelB/LevelC  
    ```  
    ![Demo-cd](sample/doc/cd.gif)  

- `gcd` : (git-smart change directory)
    ```bash
    $ gcd hierC        #Unique DIR name.
    $ gcd L            #Unique directory header character. Even if it is not LevelC, judged by L.
    ```
    ![Demo-gcd](sample/doc/gcd.gif)  
    - In this way, you can move smartly without entering your PATH.  

## Environment (develop)
1. Ubuntu 18.04.5 LTS
1. rustc 1.50.0 
1. git version 2.17.1


## Build 
- release 
    ```
    $ cargo build --release
    ./target/release/gcd
    ```
- compact release
    ```
    $ cargo rustc --release -- -C opt-level=z -C link-args=-Wl,-x,-S
    $ file ./target/release/gcd
    ./target/release/gcd: xxx,xxx, not stripped
    
    $ strip ./target/release/gcd
    $ file ./target/release/gcd
    ./target/release/gcd: xxx,xxx, stripped
    ```

## Setting for bash
1. Must be set to alias. Add to `$HOME/.bashrc`
    ```bash
    function _gcd() {
        gd=`~/<USER Build PATH>/target/release/gcd $1`;
        cd $gd;
        pwd;
    }
    alias gcd=_gcd
    ```
1. (Optional: Application to bash-completion.) Add to `$HOME/.bash_completion` 
    ```bash
    function comp_gcd() {
      COMPREPLY=( $(compgen -W "$( git rev-parse --show-toplevel | xargs -I {} cat "{}/.keepCache" | awk -F/ \{print\ \$\(NF-1\)\} | tr "\n" " ")" ${COMP_WORDS[COMP_CWORD]}  ) ) 
    }
    complete -F comp_gcd gcd
    ```


## Setting for csh  
1. Must be set to alias. Add to `$HOME/.cshrc`
    ```bash
    alias gcd 'cd `(~/<USER Build PATH>/target/release/gcd \!*)`;pwd'
    ```
1. (Optional: Application to completion.) Add to `$HOME/.cshrc` 
    ```bash
    complete gcd 'p@1@`git rev-parse --show-toplevel | xargs -I {} cat "{}/.keepCache" | awk -F/ \{print\ \$\(NF-1\)\} | tr "\n" " "`@'
    ```

## Setting common (bash & csh)
1. (Optional: Symbolic links can be used by creating a Config file.) Create a `<git root directory>/.keepSlink` file. 
    Example: The sample symbolic directory is described as follows.
    ```bash
    $ cat .keepSlink  
    ./sample/hier/route2/levelA/s_hierB/  
    ./sample/hier/route2/levelA/levelB/s_hierC/  
    ```
    - This is useful for non-Git managed DIRs.
    
## Use binary release  
- Binary is a cross-compiler release for musl-libc such as CentOS and Ubuntu.  
- Set "~/\<USER Build PATH\>/target/release/gcd" to the path where gcd is installed.  

## Other operations
1. Rebuild the dictionary `.keepCache` 
    ```bash
    $ gcd //        #Create a list and go to the repository root.
    ```
1. Create `.gitkeep` to target the current DIR.
    ```bash
    $ gcd ..        #Place ".gitkeep" in the current DIR and rebuild the dictionary.
    ```
1. Go to the root DIR in your Git repository. (Git Home Directory)
    ```bash
    $ gcd
    ```
1. Internal Automatic Operation
    1. If the creation date of the file ".git/logs/HEAD" is newer than the creation date of the dictionary list, the dictionary list ".keepCache" will be automatically rebuilt before being moved by the command.   
1. The case of the same DIR name "xyz" is handled by the unique name including the upper PATH.
    ```bash
    $ gcd hier123/xyz
    $ gcd hier456/xyz
    ```

## Status
- Release v0.2.0 : Enhancement of Issue #5.
- Release v0.1.0 : I started using commands in normal work.
- Start v0.0.0 : I was thinking of learning a non-interpreter language and considered C and Rust. I thought the Rust language would be interesting to start with; I often work in Git's Shell environment and wanted to make it easy to move DIRs that require up and down the hierarchy, so that's what I went for for my first Rust. I would like to learn how to write code that takes advantage of the features of the Rust language.

