" Syntax file for custom highlighting
if exists("b:current_syntax")
  finish
endif

syntax match MyComment /^#.*$/
syntax match Tag /^\.\w.*$/
syntax match HorizSep /^---.*$/

hi link MyComment Comment
hi Tag ctermfg=cyan guifg=#ff4200
hi HorizSep ctermfg=yellow guifg=#FFFF00

let b:current_syntax = "mysyntax"
