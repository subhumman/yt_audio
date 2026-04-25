## In short, a primitive CLI utility - a wrapper over yt-dlp
### Installation and launch
```Bash
git clone <url>
cd dowAu
cargo run --release -- "url плейлиста или же песни" "путь куда устанавливать" // тут еще надо было по идее указать формат
// но не советую, ибо придется скачивать ffmpeg, а я лично его скачивать не хотел.
```
In general, you don't really need to specify the format in which the installation will take place. In this case, everything will be installed in the webm format, which is kind of universal.

### reasons for such bluntness
I wrote it purely for myself to download tracks DARK PRINCE. I don't need convenience, I need a quick result. Something like that
