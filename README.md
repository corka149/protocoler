# protocoler

[![Rust](https://github.com/corka149/protocoler/actions/workflows/rust.yml/badge.svg)](https://github.com/corka149/protocoler/actions/workflows/rust.yml)

> A minimalistic typer for protocols

A Fast and minimalistic protocol generator built powered by Clap & Rust.
It can output the protocol in different formats.

## Usage

`protocoler` use a dialog-like approach. It supports the three different
output formats raw, csv and markdown (Jira flavour).

Here an example:
```
> $ ./protocoler                                                                                                                                                                                                             [±main ●▴]
Enter: (i) add Info, (d) add Decision, (t) add Task, (r) Remove entry, (entryId) edit entry OR (q) for Quit: 
i
---Said by:
Alice
---Note:
Foo
ID: 0
Enter: (i) add Info, (d) add Decision, (t) add Task, (r) Remove entry, (entryId) edit entry OR (q) for Quit: 
0
---Said by ['Alice']:
   
---Note ['Foo']:
Bar
Enter: (i) add Info, (d) add Decision, (t) add Task, (r) Remove entry, (entryId) edit entry OR (q) for Quit: 
t
---Said by:
Bob
---Note:
Bye bye
ID: 1
Enter: (i) add Info, (d) add Decision, (t) add Task, (r) Remove entry, (entryId) edit entry OR (q) for Quit: 
r 
---Delete an entry by ID:
1
Enter: (i) add Info, (d) add Decision, (t) add Task, (r) Remove entry, (entryId) edit entry OR (q) for Quit: 
t
---Said by:
Bob
---Note:
Let's go
ID: 2
Enter: (i) add Info, (d) add Decision, (t) add Task, (r) Remove entry, (entryId) edit entry OR (q) for Quit: 
q
Select output format [raw, markdown, csv]
csv
timestamp,entry_type,said_by,text
'2021-11-04 21:14:14.085390281 +01:00','INFO','Alice','Bar'
'2021-11-04 21:15:18.203249244 +01:00','TASK','Bob','Let's go'

```
