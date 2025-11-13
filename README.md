# HTFS - Hash Tag File System

Prefixed tag based, hierarchic file system.

<!doctype html>
<html>
  <head>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
    <meta name="Author" content="Made by 'tree'" />
    <meta
      name="GENERATOR"
      content="tree v2.2.1 © 1996 - 2024 by Steve Baker, Thomas Moore, Francesc Rocher, Florian Sesser, Kyosuke Tokoro"
    />
    <title>Directory Tree</title>
    <style type="text/css">
      BODY {
        font-family: monospace, sans-serif;
        color: black;
      }
      P {
        font-family: monospace, sans-serif;
        color: black;
        margin: 0px;
        padding: 0px;
      }
      A:visited {
        text-decoration: none;
        margin: 0px;
        padding: 0px;
      }
      A:link {
        text-decoration: none;
        margin: 0px;
        padding: 0px;
      }
      A:hover {
        text-decoration: underline;
        background-color: yellow;
        margin: 0px;
        padding: 0px;
      }
      A:active {
        margin: 0px;
        padding: 0px;
      }
      .VERSION {
        font-size: small;
        font-family: arial, sans-serif;
      }
      .NORM {
        color: black;
      }
      .FIFO {
        color: purple;
      }
      .CHAR {
        color: yellow;
      }
      .DIR {
        color: blue;
      }
      .BLOCK {
        color: yellow;
      }
      .LINK {
        color: aqua;
      }
      .SOCK {
        color: fuchsia;
      }
      .EXEC {
        color: green;
      }
    </style>
  </head>
  <body>
    <h1>Directory Tree</h1>
    <p>
      [&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;1]&nbsp;&nbsp;&nbsp;&nbsp;<a
        href="mountpointmountpoint//"
        >mountpoint/</a
      ><br />
      ├── [&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;3]&nbsp;&nbsp;&nbsp;&nbsp;<a
        href="mountpointmountpoint/%23Documents/"
        >#Documents</a
      ><br />
      │   ├── [&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;6]&nbsp;&nbsp;&nbsp;&nbsp;<a
        href="mountpointmountpoint/%23Documents/downloaded-document.pdf"
        >downloaded-document.pdf</a
      ><br />
      │   └── [&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;5]&nbsp;&nbsp;&nbsp;&nbsp;<a
        href="mountpointmountpoint/%23Documents/%23Downloads/"
        >#Downloads</a
      ><br />
      │   &nbsp;&nbsp;&nbsp; └──
      [&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;6]&nbsp;&nbsp;&nbsp;&nbsp;<a
        href="mountpointmountpoint/%23Documents/%23Downloads/downloaded-document.pdf"
        >downloaded-document.pdf</a
      ><br />
      └── [&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;2]&nbsp;&nbsp;&nbsp;&nbsp;<a
        href="mountpointmountpoint/%23Downloads/"
        >#Downloads</a
      ><br />
      &nbsp;&nbsp;&nbsp; ├──
      [&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;4]&nbsp;&nbsp;&nbsp;&nbsp;<a
        href="mountpointmountpoint/%23Downloads/%23Documents/"
        >#Documents</a
      ><br />
      &nbsp;&nbsp;&nbsp; │   └──
      [&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;6]&nbsp;&nbsp;&nbsp;&nbsp;<a
        href="mountpointmountpoint/%23Downloads/%23Documents/downloaded-document.pdf"
        >downloaded-document.pdf</a
      ><br />
      &nbsp;&nbsp;&nbsp; └──
      [&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;6]&nbsp;&nbsp;&nbsp;&nbsp;<a
        href="mountpointmountpoint/%23Downloads/downloaded-document.pdf"
        >downloaded-document.pdf</a
      ><br />
      <br /><br />
      <p>5 directories, 4 files</p>
    </p>

    <hr />
    <p class="VERSION">
      tree v2.2.1 © 1996 - 2024 by Steve Baker and Thomas Moore <br />
      HTML output hacked and copyleft © 1998 by Francesc Rocher <br />
      JSON output hacked and copyleft © 2014 by Florian Sesser <br />
      Charsets / OS/2 support © 2001 by Kyosuke Tokoro
    </p>

  </body>
</html>

###### TODO

- circumvent sqlite size limit
- handle hard links
- calculate directory size
- handle superset duplicates (e.g. $f \in A \cup B$, $f' \in A$ where $f$ and $f'$ has the same name)
