/* context.S - Low-level context setup (AT&T syntax) */

    .section .text
        .global load_gdt
            .type load_gdt, @function
            /* void load_gdt(void* gdt_ptr) */
            load_gdt:
                lgdt (%rdi)        /* Load GDT pointer from argument */
                    mov $0x10, %ax     /* Kernel data selector */
                        mov %ax, %ds
                            mov %ax, %es
                                mov %ax, %ss
                                    ljmp $0x08, $next  /* Far jump to reload CS */
                                    next:
                                        ret

                                            .global load_idt
                                                .type load_idt, @function
                                                /* void load_idt(void* idt_ptr) */
                                                load_idt:
                                                    lidt (%rdi)        /* Load IDT pointer */
                                                        ret

                                                            .global enable_interrupts
                                                                .type enable_interrupts, @function
                                                                enable_interrupts:
                                                                    sti
                                                                        ret

                                                                            .global disable_interrupts
                                                                                .type disable_interrupts, @function
                                                                                disable_interrupts:
                                                                                    cli
                                                                                        ret

                                                                                            .global halt_cpu
                                                                                                .type halt_cpu, @function
                                                                                                halt_cpu:
                                                                                                    hlt
                                                                                                        ret