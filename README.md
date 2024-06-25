# BigStr
A command line tool to make string BIG

## Example
```shell
$ echo "BIGSTR" | bigstr -f 'DejaVuSansM Nerd Font' -s 17 --background '=' --offset -0.2
========================================================================================
========================================================================================
========================================================================================
==╭──────────╮===╭──────────╮====╭────────╮===╭─────────╮╭────────────────────────╮=====
==│  ╭─────╮ ╰╮==╰───╮  ╭───╯==╭─╯╭───────╯=╭─╯╭────────╯╰─────╮  ╭─────╮  ╭─────╮╰─╮===
==│  │=====╰╮ │======│  │=====╭╯ ╭╯=========│  │===============│  │=====│  │=====│  │===
==│  │=====╭╯ │======│  │=====│ ╭╯==========│  ╰╮==============│  │=====│  │=====│  │===
==│  ╰─────╯ ╭╯======│  │====╭╯ │===========╰─╮ ╰─────╮========│  │=====│  ╰─────╯ ╭╯===
==│  ╭─────╮ ╰╮======│  │====│  │====╭─────╮==╰─────╮ ╰─╮======│  │=====│         ╭╯====
==│  │=====╰╮ ╰╮=====│  │====╰╮ ╰╮===╰──╮  │========╰─╮ ╰╮=====│  │=====│  ╭────╮ ╰╮====
==│  │======│  │=====│  │=====│  │======│  │==========│  │=====│  │=====│  │====╰─╮╰╮===
==│  │====╭─╯ ╭╯=====│  │=====╰╮ ╰─╮====│  │╭──╮=====╭╯ ╭╯=====│  │=====│  │======│ ╰╮==
==│  ╰────╯ ╭─╯==╭───╯  ╰───╮==╰──╮╰────╯╭─╯╰╮ ╰─────╯╭─╯======│ ╭╯=====│ ╭╯======╰╮ ╰╮=
==╰─────────╯====╰──────────╯=====╰──────╯===╰────────╯========╰─╯======╰─╯========╰──╯=
========================================================================================
========================================================================================
========================================================================================


$ bigstr -f Arial -s 12 -m BigsTR
                                                                     
                                                                     
 ╭────────╮    ╭─╮                       ╭─────────╮   ╭─────────╮   
 │ ╭─────╮╰╮   ╰─╯                       ╰───╮ ╭───╯   │╭───────╮╰╮  
 │ │     │ │   ╭─╮   ╭─╮ ╭───╮ ╭──╮╭──╮      │ │       ││      ╭╯╭╯  
 │ ╰─────╯ │   │ │  ╭╯╭╯ ╰─╮ │ │ ╭╯╰──╯      │ │       │╰──────╱─╯   
 │ ╭──────╮╰╮  │ │  │╭╯    │ │ ╰─╲────╮      │ │       │╭────╮ ╰╮    
 │ │      │ │  │ │  │╰╮   ╭╯ │ ╭─╱───╮╰╮     │ │       ││    ╰─╮╰╮   
 │ ╰──────╱─╯  │ │  ╰─╲───╯  │ ╰╮╰───╯╭╯     │ │       ││      ╰╮╰╮  
 ╰────────╯    ╰─╯  ╭─╱───╮ ╭╯  ╰─────╯      ╰─╯       ╰╯       ╰─╯  
                    ╰╮╰───╯╭╯                                        
                     ╰─────╯                                         
```


## Usage
```
$ bigstr --help
A command-line tool to make string BIG

Usage: bigstr [OPTIONS] [COMMAND]

Commands:
  list  show list of available fonts
  help  Print this message or the help of the given subcommand(s)

Options:
  -f, --famliy <FAMLIY>
          A name of font famliy

      --font-file <FONT_FILE>
          A path to a font file

  -m, --message <MESSAGE>
          A message to render, if not specified, read from stdin

  -s, --size <SIZE>
          The height of the rendered text
          
          [default: 18]

  -o, --offset <OFFSET>
          Each character will be offseted by this value
          
          i.e. -1.0 means the character will be moved 100% of its width to the left,
          
          0.0 means no offset,
          
          1.0 means the character will be moved 100% of its width to the right.
          
          Giving values like -0.5 will make the characters overlap a little. Have Fun!
          
          [default: 0.0]

  -t, --threshold <THRESHOLD>
          The threshold to determine whether a pixel is foreground or background
          
          [default: 0.5]

  -b, --background <BACKGROUND>
          The background character
          
          [default: " "]

      --mode <MODE>
          The mode to render the text
          
          "round": the rendered text will be rounded
          
          "square": the rendered text will be squared
          
          default is "round"
          
          [default: round]

  -a, --aspect-ratio <ASPECT_RATIO>
          The aspect ratio of the cursor
          
          The ratio height/width of actual rendered character (e.g. the unicode box drawing characters)
          
          Since we use (mono) character as pixel, the proper aspect ratio should be given to make the image looks good
          
          [default: 2.0]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```