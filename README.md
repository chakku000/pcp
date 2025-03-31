# pcp
pcpはフィイルやディレクトリを指定して、ファイル名とファイル内容をコードブロックで出力するツールです。
複数のファイルをLLMに内容をコピペが少しだけ楽になります。
pcpの出力をそのまま`pbpaste`などのクリップボードツールに流してしまうのがおすすめです。

例えば `a.txt`というファイルを指定すると以下のように出力します。
````
$ pcp a.txt
```a.txt
This is the contents of a.txt
```
````

複数のファイルを指定できます。
````
$ pcp a.txt b.txt
```a.txt
This is the contents of a.txt
```
```b.txt
This is the contents of b.txt
```
````

ディレクトリを指定した場合は、ディレクトリの配下のファイルを再帰的に探します。

````
$ tree src
src
├── lib.rs
└── main.rs

$ pcp src
```src/lib.rs
This is contents of src/lib.rs
```
```src/main.rs
This is contents of src/main.rs
```
````

## ビルドとインストール
リポジトリをcloneしたら以下のコマンドでビルドして`PATH`の通っているディレクトリに配置してください。
```
$ cargo build --release
$ mv target/release/pcp $HOME/.local/bin
```
`$HOME/.local/bin`はパスの通っている任意のディレクトリに読み替えてください。
