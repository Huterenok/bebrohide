# Bebrohide CLI

Bebrohide CLI is a tool that allows you to hide encoded file inside another one. If you want to reveal your hidden file, you can do it by typing your password for encoding/decoding.

## Installation

```shell
cargo install bebrohide_cli
```

## Getting Started

To get info about cli, just type:

```shell
bebrohide help
```

or to get detailed info about cli:

```shell
bebrohide help <COMMAND>
```

## Commands

## `bebrohide hide`

Start by thinking about which file you want to hide in another. For example, i have a text file 'bebra.txt' and i want to hide it inside my binary file 'banan.exe'. Firstly, i need to type in my terminal:

```shell
bebrohide hide --source-file ./banan.exe --hidden-file ./bebra.txt
```

or shorter:

```shell
bebrohide hide -s ./banan.exe -r ./bebra.txt
```

Also you can use optional parameter "--hide_inside" to specify path to file where result will be. By default it is source file:

```shell
bebrohide hide --source-file ./banan.exe --hidden-file ./bebra.txt --hide-inside ./mega_bebra.exe
```

or

```shell
bebrohide hide -s ./banan.exe -r ./bebra.txt -i ./mega_bebra.exe
```

Now you have file in your folder named 'mega_bebra.exe' and it can still be executed.

## `bebrohide check`

You can check if a file has any hidden files inside it by typing:

```shell
bebrohide check --hidden-file ./mega_bebra.exe
```

or

```shell
bebrohide check -r ./mega_bebra.exe
```

Returns 'true' or 'false' depending on whether the file has a hidden one inside.

## `bebrohide reveal`

Now you want to read your file from the hidden one. Let's decode it inside folder 'assets':

```shell
bebrohide reveal --hidden-file ./mega_bebra.exe --destination-folder ./assets
```

or

```shell
bebrohide reveal -r ./mega_bebra.exe -d ./assets
```

Also you can use optional parameter "--clear" to clear source file from hidden one:

```shell
bebrohide reveal --hidden-file ./mega_bebra.exe --destination-folder ./assets --clear
```

or

```shell
bebrohide reveal -r ./mega_bebra.exe -d ./assets -c
```

And now you have your decoded file 'bebra.txt' inside 'assets' folder. If you use '--clear' parameter, file 'mega_bebra.exe' will be cleared of hidden one.
