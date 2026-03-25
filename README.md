Lexer dla języka Lox, który przekształca kod źródłowy w sekwencję tokenów, a następnie generuje plik .html z pokolorowaną składnią.

Użycie:
```
cargo run . -- nazwapliku.lox
```
Program wygeneruje plik nazwapliku.lox.html w tej samej ścieżce

| Nazwa tokena        | Id tokena           | Regex                      |
|---------------------|---------------------|----------------------------|
| Identyfikator       | Identifier          | `[a-zA-Z][a-zA-Z0-9]*`     |
| Liczba              | NumberLiteral       | `[0-9]+`                   |
| String              | StringLiteral       | `"[^"]*"`                  |
| Komentarz           | Comment             | `//.*`                     |
| Znak dzielenia      | Slash               | `/`                        |
| Znak mnożenia       | Star                | `\*`                       |
| Znak plus           | Plus                | `\+`                       |
| Znak minus          | Minus               | `-`                        |
| Średnik             | Semicolon           | `;`                        |
| Przecinek           | Comma               | `,`                        |
| Spacja              | Space               | `[ \t]+`                   |
| Znak równości       | Equal               | `=`                        |
| Lewy nawias         | OpenParentheses     | `\(`                       |
| Prawy nawias        | CloseParentheses    | `\)`                       |
| Lewa klamra         | OpenBrace           | `{`                        |
| Prawa klamra        | CloseBrace          | `}`                        |
| Równość porównania  | DoubleEqual         | `==`                       |
| Wykrzyknik          | Bang                | `!`                        |
| Nierówność          | BangEqual           | `!=`                       |
| Mniejszy            | OpenAngleBracket    | `<`                        |
| Większy             | CloseAngleBracket   | `>`                        |
| Mniejsze lub równe  | LessEqual           | `<=`                       |
| Większe lub równe   | GreaterEqual        | `>=`                       |
| Kropka              | Dot                 | `\.`                       |
| AND                 | And                 | `and`                      |
| class               | Class               | `class`                    |
| else                | Else                | `else`                     |
| false               | False               | `false`                    |
| for                 | For                 | `for`                      |
| fun                 | Fun                 | `fun`                      |
| if                  | If                  | `if`                       |
| nil                 | Nil                 | `nil`                      |
| or                  | Or                  | `or`                       |
| print               | Print               | `print`                    |
| return              | Return              | `return`                   |
| super               | Super               | `super`                    |
| this                | This                | `this`                     |
| true                | True                | `true`                     |
| var                 | Var                 | `var`                      |
| while               | While               | `while`                    |
| Nowa linia          | NewLine             | `\n`                       |
| Koniec pliku        | EOF                 | `$`                        |