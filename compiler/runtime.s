.text
.globl main
msg:
    .string "Hello world!\n"

r2: # 1
    movq %r8, %rax
    ret

main: # 0
    call print_msg
    movq $0, %rax
    ret
    
    movq r2, %r8
    movq $3, %r9
    movq $4, %r10
    jmp f5

f5: # 3
    movq %r10, %r11
    subq $2, %r11
    movq %r9, %r10
    addq $1, %r10
    movq %r10, %r9
    imulq %r11, %r9
    movq %r8, %r11
    movq %r9, %r8
    jmp *%r11

print_msg:
    # write(1, message, 13)
    movq $1, %rax            # system call 1 is write
    movq $1, %rdi            # file handle 1 is stdout
    movq $msg, %rsi          # address of string to output
    movq $13, %rdx           # number of bytes
    syscall            # invoke operating system to do the write
    ret
