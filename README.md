<link href="./README.css" rel="stylesheet"></link>

# Shikibu
Shikibu is a Japanese programming language.  
It was created for Japanese beginners in programming.  
Now, let's try programming with Shikibu!

## Sample
<pre class="shikibu"><code><span class="comment">＃　変数定義</span>
<span class="identifier">値</span><span class="symbol">＝</span><span class="number">１</span>

<span class="keyword">表示</span>　<span class="identifier">値</span>
<span class="comment">＃　出力：　1</span>

<span class="comment">＃　関数定義</span>
<span class="keyword">関数</span>　<span class="function">否定</span>（<span class="identifier">真偽値</span>）
　　<span class="keyword">もどす</span>　<span class="identifier">真偽値</span><span class="symbol">＝＝</span><span class="keyword">偽</span>

<span class="keyword">表示</span>　<span class="function">否定</span>（<span class="keyword">真</span>）
<span class="comment">＃　出力：　偽</span>

<span class="comment">＃　条件分岐</span>
<span class="keyword">もし</span>　<span class="identifier">値</span><span class="symbol">＝＝</span><span class="number">２</span>　<span class="keyword">なら</span>
　　<span class="keyword">表示</span>　<span class="string">「値は２です」</span>
<span class="keyword">もしくは</span>
　　（<span class="identifier">値</span><span class="symbol">＜</span><span class="number">２</span>　<span class="keyword">または</span>　<span class="number">２</span><span class="symbol">＜</span><span class="identifier">値</span>）
　　<span class="keyword">かつ</span>　<span class="number">０</span><span class="symbol">＜</span><span class="identifier">値</span>
<span class="keyword">なら</span>
　　<span class="keyword">表示</span>　<span class="string">「値は２ではない正の数です」</span>
<span class="keyword">ちがえば</span>
　　<span class="keyword">表示</span>　<span class="string">「値が不正です」</span>
<span class="comment">＃　出力：　値は２ではない正の数です</span>


<span class="comment">＃　ループ</span>
<span class="identifier">回数</span><span class="symbol">＝</span><span class="number">０</span>
<span class="keyword">くりかえし</span>
　　<span class="identifier">回数</span><span class="symbol">＝</span><span class="identifier">回数</span><span class="symbol">＋</span><span class="number">１</span>
　　<span class="keyword">もし</span>　<span class="identifier">回数</span><span class="symbol">＜</span><span class="number">７</span>　<span class="keyword">なら</span>
　　　　<span class="keyword">つぎへ</span>
　　<span class="keyword">もしくは</span>　<span class="number">９</span><span class="symbol">＝＝</span><span class="identifier">回数</span>　<span class="keyword">なら</span>
　　　　<span class="keyword">ぬける</span>
　　<span class="keyword">表示</span>　<span class="string">「現在の回数は」</span><span class="symbol">＋</span><span class="identifier">回数</span><span class="symbol">＋</span><span class="string">「です」</span>
<span class="comment">＃　出力：　現在の回数は７です</span>
<span class="comment">＃　出力：　現在の回数は８です</span></code></pre>
