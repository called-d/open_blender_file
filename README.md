# open_blender_file.exe

Open .blend file with the last saved version of the Blender.exe .

adapted from [ファイルを作成したバージョンを判別してBlenderを起動するWindowsバッチファイル - yoitaro’s blog](https://yoitaro.hatenablog.com/entry/2019/11/10/224757)

---
.blend ファイルをダブルクリックで開くときに自動で Blender のバージョンを選ぶプログラムです。コア・コンセプトは上記ブログ記事のとおりです。

.blend ファイルには、最後に編集（保存）した Blender.exe のバージョン番号が書き込まれているので、その情報を利用しています。

## 設定 (Windows 11)
.blend ファイルを右クリックしてメニューを開き
`プログラムから開く` → `別のプログラムを選択`

`アプリを選択して .blend ファイルを開く` のウィンドウで `PCでアプリを選択する` から 実行ファイル open_blender_file.exe を選ぶ

## 使われる Blender.exe の優先順
1. 設定ファイル(`%APPDATA%\open_blender_file\config.json`)に該当するバージョンがあればそれを開く
2. 設定ファイルに `default: "3.6"` のように設定されており、設定ファイルに該当バージョンがあり、ファイルの最終更新バージョンよりも新しければそれを開く
3. `C:\Program Files\Blender Foundation\Blender <version>\` を探して該当するバージョンの実行ファイルが見つかればそれを開く
4. 上記すべて該当しない場合、開かない。

### 設定ファイルの例
```json5:config.json
{
    "executable_map":{
        // ショートカットでも可
        "2.93": "C:\\Users\\John_Smith\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Blender\\Blender 2.93.lnk",
        "3.4": "C:\\Program Files\\Blender Foundation\\Blender 3.4\\blender-launcher.exe",
        "4.1": "C:\\Program Files\\Blender Foundation\\Blender 4.1\\blender-launcher.exe",
    },
    // "default": "4.1", のように書く
    "default": null,
}
```

## コマンドライン引数
```
Usage: open_blender_file <FILE> [options] ["--" [extra args for blender.exe]]

Options:
        --set-icon      set icon (registry editing).
    -h, --help          print this help menu
    -p, --print-version
                        print version and exit.
        --dry-run       print found blender executable and exit.
```

### --set-icon について
レジストリの値は正しく書けているように見えるので、何かでリフレッシュがかかったタイミングでアイコンが反映されます
#### なぜレジストリの値を直接変更する方式にしたのですか
* Blender 本体から .ico のリソースを持ってくるのはあまりやりたくない
  + 管理者権限でレジストリの値を弄るよりもやりたくない
* windres-rs のクロスコンパイルがつらかった[^1]

[^1]: https://users.rust-lang.org/t/compile-for-windows-from-linux-when-have-build-rs/76858/11
