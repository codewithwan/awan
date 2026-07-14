# awan shell integration (zsh)
#
# Emits `cmd.start`, `cmd.ok`, and `cmd.failed` events as you run commands, so
# a running `awan watch` reacts to your shell in real time.
#
# Setup (run the companion in a dedicated tmux/zellij pane so it doesn't fight
# your working terminal):
#
#   export AWAN_PIPE="$HOME/.awan.pipe"
#   mkfifo "$AWAN_PIPE"
#   awan watch --pipe "$AWAN_PIPE"        # in a separate pane
#
# Then in your ~/.zshrc:
#
#   export AWAN_PIPE="$HOME/.awan.pipe"
#   source /path/to/awan/shell/awan.zsh
#
# Writes are non-blocking: if no companion is listening, events are dropped
# and your prompt is never held up.

: "${AWAN_PIPE:=$HOME/.awan.pipe}"

zmodload zsh/system 2>/dev/null

_awan_send() {
  [[ -p "$AWAN_PIPE" ]] || return
  local fd
  # open the fifo write-only, non-blocking; skip if nobody is reading
  sysopen -wo nonblock fd "$AWAN_PIPE" 2>/dev/null || return
  print -u "$fd" -r -- "$1" 2>/dev/null
  exec {fd}>&-
}

autoload -Uz add-zsh-hook

_awan_running=0
_awan_preexec() { _awan_running=1; _awan_send cmd.start }
_awan_precmd() {
  local ec=$?
  (( _awan_running )) || return          # nothing ran (fresh prompt)
  _awan_running=0
  (( ec == 0 )) && _awan_send cmd.ok || _awan_send cmd.failed
}

add-zsh-hook preexec _awan_preexec
add-zsh-hook precmd _awan_precmd
