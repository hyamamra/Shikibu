# Shikibu
Shikibu is a Japanese programming language.  
It was created for Japanese beginners in programming.  
Now, let's try programming with Shikibu!

## Sample
```
＃　変数定義
値＝１

表示　値
＃　出力：　1

＃　関数定義
しごと　否定（真偽値）
　　かえす　真偽値＝＝偽

表示　否定（真）
＃　出力：　偽

＃　条件分岐
もし　値＝＝２　なら
　　表示　「値は２です」
ちがえば　もし
　　（値＜２　または　２＜値）
　　かつ　０＜値
なら
　　表示　「値は２ではない正の数です」
ちがえば
　　例外　「値が不正なため処理を終了します」
＃　出力：　値は２ではない正の数です


＃　ループ
回数＝０
くりかえし
　　回数＝回数＋１
　　もし　回数＜７　なら
　　　　つぎへ
　　ちがえば　もし　９＝＝回数　なら
　　　　ぬける
　　表示　「現在の回数は」＋回数＋「です」
＃　出力：　現在の回数は７です
＃　出力：　現在の回数は８です
```
