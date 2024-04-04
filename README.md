# bkmk
bookmark directories

## Usage

```shell
$ cd ~/some_directory
$ bkmk mydir
Creating bookmark mydir.
Wrote state to /Users/christopher.olin/.bkmk
$ cd 
$ pwd
/Users/<your.username>
$ bkmk -l
Current bookmarks:
mydir -> /Users/<your.username>/some_directory
$ cd $(bkmk -f mydir)
$ pwd
/Users/<your.username>/some_directory
$ bkmk -r mydir
Removing bookmark mydir.
Wrote state to /Users/christopher.olin/.bkmk
```

I recommend putting the following in your .zshrc or .bashrc:

```zsh
goto() {
    local destination=`bkmk -f $1`
    if [[ "$destination" == "" ]]; then
        return 1
    fi
    cd $destination
}
```

This will allow for:

```shell
$ cd ~/some_directory
$ bkmk mydir
Creating bookmark mydir.
Wrote state to /Users/christopher.olin/.bkmk
$ cd 
$ pwd
/Users/<your.username>
$ goto mydir
$ pwd
/Users/<your.username>/some_directory
```
