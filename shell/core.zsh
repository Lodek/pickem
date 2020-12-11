function pickem-from-file() {
   LBUFFER="${LBUFFER}$(pickem $1)"
   local ret=$?
   zle reset-prompt
   return $ret
}
