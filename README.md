# OXY
## Ordinary Xenial Yammerer

Another golfing, stack-based language.

## Operators

### `0` `1` `2` `3` `4` `5` `6` `7` `8` `9` - Push Digit

Pushes the corresponding digit to the stack.

### `Λ` - Map

Maps the inner code `Λ...°` onto a list.

List `a` &rarr; `a.map(...)`.

### `†` - Seedless Fold

Folds the inner code `Λ...°` onto a list.

List `a` &rarr; `a.fold(...)`.

### `‡` - Seeded Fold

Folds the inner code `Λ...°` onto a list.

List `a`, Any `b` &rarr; `a.fold(b, ...)`.

### `°` - Close

Closes deepest open structure.

### `¡` - Singleton

Wraps the top of the stack in a list.

Any `a` &rarr; `[a]`.

### `⍨` - Swap Top Two

Swaps the position of the top two elements on the stack.

Any `a`, `b` &rarr; `b`, `a`.

### `‿` - Pair

Wraps the top two elements of the stack in a list.

Any `a`, `b` &rarr; `[a b]`.

### `‾` - Wrap Stack

Wraps the entire stack in a list.

All `a0`, `a1`, …, `aN` &rarr; `[a0 a1 … aN]`.

### `…` - Unwrap Stack

Dumps all elements in a list to the stack.

List `[a0 a1 … aN]` &rarr; `a0`, `a1`, …, `aN`.

### `+` - Add

Integers `a`, `b` &rarr; `a+b`. (Vectorizes.)

### `-` - Subtract

Integers `a`, `b` &rarr; `a-b`. (Vectorizes.)

### `×` - Multiply

Integers `a`, `b` &rarr; `a×b`. (Vectorizes.)

### `÷` - Divide

Integers `a`, `b` &rarr; `a÷b`. (Vectorizes.)

### `↓` - Interleave

Given two lists, pushes them interleaved (flattened zip).

Lists `a0…aN`, `b0…bN` &rarr; `[a0 b0 a1 b1 … aN bN]`.

### `↑` - Uninterleave

Given a list, pushes two lists, each containing every other element at different offsets.

List `a0…aN` &rarr; `a0 a2 a4…`, `a1 a3 a54…`.

### `c` - Concatenate

Concatenates two entries together. Non-lists are prepended/appended respectively.

### `r` - One Range

Integer `a` &rarr; range from 1 to `a`.

## Codepage

|      |  `_0`  |  `_1`  |  `_2`  |  `_3`  |  `_4`  |  `_5`  |  `_6`  |  `_7`  |  `_8`  |  `_9`  |  `_A`  |  `_B`  |  `_C`  |  `_D`  |  `_E`  |  `_F`  |
|------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|
| `0_` |  `…`   |  `‾`   |  `‿`   |  `¡`   |  `¿`   |  `·`   |  `∧`   |  `∨`   |  `≔`   |  `§`   |  `¶`   |  `∀`   |  `∃`   |  `◯`   |  `√`   |  `¬`   |
| `1_` |  `×`   |  `÷`   |  `≠`   |  `≈`   |  `≤`   |  `≥`   |  `±`   |  `⊓`   |  `⊔`   |  `∈`   |  `∞`   |  `∅`   |  `⊆`   |  `⊇`   |  `∩`   |  `∪`   |
| `2_` |  ` `   |  `!`   |  `"`   |  `#`   |  `$`   |  `%`   |  `&`   |  `'`   |  `(`   |  `)`   |  `*`   |  `+`   |  `,`   |  `-`   |  `.`   |  `/`   |
| `3_` |  `0`   |  `1`   |  `2`   |  `3`   |  `4`   |  `5`   |  `6`   |  `7`   |  `8`   |  `9`   |  `:`   |  `;`   |  `<`   |  `=`   |  `>`   |  `?`   |
| `4_` |  `@`   |  `A`   |  `B`   |  `C`   |  `D`   |  `E`   |  `F`   |  `G`   |  `H`   |  `I`   |  `J`   |  `K`   |  `L`   |  `M`   |  `N`   |  `O`   |
| `5_` |  `P`   |  `Q`   |  `R`   |  `S`   |  `T`   |  `U`   |  `V`   |  `W`   |  `X`   |  `Y`   |  `Z`   |  `[`   |  `\`   |  `]`   |  `^`   |  `_`   |
| `6_` | &#96;  |  `a`   |  `b`   |  `c`   |  `d`   |  `e`   |  `f`   |  `g`   |  `h`   |  `i`   |  `j`   |  `k`   |  `l`   |  `m`   |  `n`   |  `o`   |
| `7_` |  `p`   |  `q`   |  `r`   |  `s`   |  `t`   |  `u`   |  `v`   |  `w`   |  `x`   |  `y`   |  `z`   |  `{`   | &#124; |  `}`   |  `~`   |  `⌂`   |
| `8_` |  `⁅`   |  `⁆`   |  `⟦`   |  `⟧`   |  `⟨`   |  `⟩`   |  `⟪`   |  `⟫`   |  `⌈`   |  `⌉`   |  `⌊`   |  `⌋`   |  `⦇`   |  `⦈`   |  `«`   |  `»`   |
| `9_` |  `™`   |  `Ạ`   |  `ß`   |  `Ċ`   |  `Ď`   |  `Ẹ`   |  `Ψ`   |  `Ġ`   |  `Ħ`   |  `İ`   |  `Ĵ`   |  `Ķ`   |  `Ŀ`   |  `Λ`   |  `Ŋ`   |  `Ọ`   |
| `A_` |  `Þ`   |  `Ω`   |  `Ř`   |  `Ŝ`   |  `Ť`   |  `Ụ`   |  `Δ`   |  `Ẅ`   |  `Ξ`   |  `Ỵ`   |  `Ż`   |  `∁`   |  `°`   |  `∫`   |  `∏`   |  `∑`   |
| `B_` |  `θ`   |  `ạ`   |  `β`   |  `ċ`   |  `đ`   |  `ė`   |  `φ`   |  `ġ`   |  `ħ`   |  `ï`   |  `ĵ`   |  `ķ`   |  `ŀ`   |  `μ`   |  `ñ`   |  `ō`   |
| `C_` |  `π`   |  `ω`   |  `ř`   |  `ŝ`   |  `ŧ`   |  `λ`   |  `μ`   |  `ŵ`   |  `χ`   |  `ÿ`   |  `ż`   |  `⊢`   |  `⍨`   |  `†`   |  `‡`   |  `↭`   |
| `D_` |  `𝔽`   |  `ℕ`   |  `ℚ`   |  `𝕊`   |  `ℤ`   |  `⊛`   |  `⊖`   |  `⊗`   |  `⊙`   |  `⊕`   |  `½`   |  `¼`   |  `¾`   |  `↋`   |  `↊`   |  `૪`   |
| `E_` |  `₀`   |  `₁`   |  `₂`   |  `₃`   |  `₄`   |  `₅`   |  `₆`   |  `₇`   |  `₈`   |  `₉`   |  `→`   |  `←`   |  `↓`   |  `↑`   |  `↔`   |  `↕`   |
| `F_` |  `⁰`   |  `¹`   |  `²`   |  `³`   |  `⁴`   |  `⁵`   |  `⁶`   |  `⁷`   |  `⁸`   |  `⁹`   |  `⇒`   |  `⇐`   |  `⇓`   |  `⇑`   |  `⇔`   |  `⇕`   |

