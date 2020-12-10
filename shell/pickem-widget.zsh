function widget-pickem-from-filename() {
   LBUFFER="${LBUFFER}$(cat ~/.pickem/shell.yml | pickem)"
   local ret=$?
   zle reset-prompt
   return $ret
}

zle -N widget-pickem-from-filename
bindkey '^N' widget-pickem-from-filename
