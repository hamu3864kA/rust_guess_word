# icedクレートを用いたGUIの実装

```zsh
cargo add iced@0.4.2 --features async-std
```

`State` 、 `State` を参照して表示する `View` 、 `View` から発生した `Message` を処理して `State` を更新する。  
React とかモダンなフロントエンドフレームワークに近いのだろうか

以下のエラーが出た。

> error: rustc 1.80.0 is not supported by the following package:
>   twox-hash@2.0.0 requires rustc 1.81
> Either upgrade rustc or select compatible dependency versions with
> `cargo update <name>@<current-ver> --precise <compatible-ver>`
> where `<compatible-ver>` is the latest version supporting rustc 1.80.0

Rustコンパイラのバージョンが適切でないらしい。

```zsh
$ rustup update
$ guess_word % rustc --version
rustc 1.82.0 (f6e511eec 2024-10-15)
```

`rustup` を使って `rustc` を更新。
