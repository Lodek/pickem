function pickem-from-file() {
   LBUFFER="${LBUFFER}$(cat $1 | pickem - )"
   local ret=$?
   zle reset-prompt
   return $ret
}

function pickem-shell() {
    pickem-from-file ~/.pickem/shell.yml
}

zle -N pickem-shell 
bindkey '^N' pickem-shell
