  .text
  .intel_syntax noprefix
  .globl main
main:
.LBL0:
  push rbp
  mov rbp, rsp
  sub rsp, 16
  mov dword ptr [rbp-12], 0
  mov dword ptr [rbp-8], 1
  mov dword ptr [rbp-4], 2
  mov eax, dword ptr [rbp-8]
  mov ecx, dword ptr [rbp-4]
  add eax, ecx
  add rsp, 16
  pop rbp
  ret 
