function pickem-from-file() {
   LBUFFER="${LBUFFER}$(cat $1 | pickem - )"
   local ret=$?
   zle reset-prompt
   return $ret
}
