.intel_syntax noprefix
.global main, add

add:
  add rsi, rdi
  mov rax, rsi
  ret

main:
  mov rdi, 3
  mov rsi, 4
  call add
  ret
