function pickem-shell() {
    pickem-from-file ~/.pickem/shell.yml
}

zle -N pickem-shell 
bindkey '^N' pickem-shell
