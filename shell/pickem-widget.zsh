function pickem-widget() {
   LBUFFER="${LBUFFER}$(cat example.yml | cargo run)"
   local ret=$?
   zle reset-prompt
   return $ret
}

zle -N pickem-widget
bindkey '^N' pickem-widget
