# pcp
pcpはフィアルを指定して、LLMに食わせる形式で出力するツールです。
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
