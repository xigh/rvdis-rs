### RISCV RV32/64 very simple disassembler

```shell
> cargo run .\elf-samples\c\0001-empty\a.out.32
```

```text
register_fini:
    00010074:    li     a5, 0
    00010078:    beq    a5, zero, 16
    0001007c:    lui    a0, 0x10000
    00010080:    addi   a0, a0, 1076
    00010084:    j      0
    00010088:    ret
_start:
// entry-point
    0001008c:    auipc  gp, 8192
    00010090:    addi   gp, gp, 3300
    00010094:    addi   a0, gp, 3124
    00010098:    addi   a2, gp, 3152
    0001009c:    sub    a2, a2, a0
    000100a0:    li     a1, 0
    000100a4:    jal    ra, 0
    000100a8:    auipc  a0, 0
    000100ac:    addi   a0, a0, 888
    000100b0:    beq    a0, zero, 16
    000100b4:    auipc  a0, 0
    000100b8:    addi   a0, a0, 896
    000100bc:    jal    ra, 0
    000100c0:    jal    ra, 0
    000100c4:    lw     a0, 0(sp)
    000100c8:    addi   a1, sp, 4
    000100cc:    li     a2, 0
    000100d0:    jal    ra, 0
    000100d4:    j      0
__do_global_dtors_aux:
    000100d8:    addi   sp, sp, 4080
    000100dc:    sw     0(sp), fp
    000100e0:    lbu    a5, 3124(gp)
    000100e4:    sw     0(sp), a2
    000100e8:    bne    a5, zero, 36
    000100ec:    li     a5, 0
    000100f0:    beq    a5, zero, 20
    000100f4:    lui    a0, 0x11000
    000100f8:    addi   a0, a0, 1376
    000100fc:    auipc  ra, 0
    00010100:    jalr   ra, 0(zero)
    00010104:    li     a5, 1
    00010108:    sb     0(gp), s4
    0001010c:    lw     ra, 12(sp)
    00010110:    lw     fp, 8(sp)
    00010114:    addi   sp, sp, 16
    00010118:    ret
frame_dummy:
    0001011c:    li     a5, 0
    00010120:    beq    a5, zero, 24
    00010124:    lui    a0, 0x11000
    00010128:    addi   a1, gp, 3128
    0001012c:    addi   a0, a0, 1376
    00010130:    auipc  t1, 0
    00010134:    jalr   zero, 0(zero)
    00010138:    ret
main:
    0001013c:    addi   sp, sp, 4080
    00010140:    sw     0(sp), a2
    00010144:    addi   fp, sp, 16
    00010148:    li     zero, 0
    0001014c:    lw     fp, 12(sp)
    00010150:    addi   sp, sp, 16
    00010154:    ret
exit:
    00010158:    addi   sp, sp, 4080
    0001015c:    li     a1, 0
    00010160:    sw     0(sp), fp
    00010164:    sw     0(sp), a2
    00010168:    addi   fp, a0, 0
    0001016c:    jal    ra, 0
    00010170:    lw     a0, 3112(gp)
    00010174:    lw     a5, 60(a0)
    00010178:    beq    a5, zero, 8
    0001017c:    jalr   ra, 0(a5)
    00010180:    addi   a0, fp, 0
    00010184:    jal    ra, 0
__libc_init_array:
    00010188:    addi   sp, sp, 4080
    0001018c:    sw     0(sp), fp
    00010190:    sw     0(sp), zero
    00010194:    lui    fp, 0x11000
    00010198:    lui    s2, 0x11000
    0001019c:    addi   a5, fp, 1380
    000101a0:    addi   s2, s2, 1380
    000101a4:    sub    s2, s2, a5
    000101a8:    sw     0(sp), a2
    000101ac:    sw     0(sp), tp
    000101b0:    srai   s2, s2, 2
    000101b4:    beq    s2, zero, 32
    000101b8:    addi   fp, fp, 1380
    000101bc:    li     s1, 0
    000101c0:    lw     a5, 0(fp)
    000101c4:    addi   s1, s1, 1
    000101c8:    addi   fp, fp, 4
    000101cc:    jalr   ra, 0(a5)
    000101d0:    bne    s2, s1, 8176
    000101d4:    lui    fp, 0x11000
    000101d8:    lui    s2, 0x11000
    000101dc:    addi   a5, fp, 1380
    000101e0:    addi   s2, s2, 1388
    000101e4:    sub    s2, s2, a5
    000101e8:    srai   s2, s2, 2
    000101ec:    beq    s2, zero, 32
    000101f0:    addi   fp, fp, 1380
    000101f4:    li     s1, 0
    000101f8:    lw     a5, 0(fp)
    000101fc:    addi   s1, s1, 1
    00010200:    addi   fp, fp, 4
    00010204:    jalr   ra, 0(a5)
    00010208:    bne    s2, s1, 8176
    0001020c:    lw     ra, 12(sp)
    00010210:    lw     fp, 8(sp)
    00010214:    lw     s1, 4(sp)
    00010218:    lw     s2, 0(sp)
    0001021c:    addi   sp, sp, 16
    00010220:    ret
memset:
    00010224:    li     t1, 15
    00010228:    addi   a4, a0, 0
    0001022c:    bgeu   t1, a2, 60
    00010230:    andi   a5, a4, 15
    00010234:    bne    a5, zero, 160
    00010238:    bne    a1, zero, 132
    0001023c:    andi   a3, a2, 4080
    00010240:    andi   a2, a2, 15
    00010244:    add    a3, a3, a4
    00010248:    sw     0(a4), zero
    0001024c:    sw     0(a4), tp
    00010250:    sw     0(a4), fp
    00010254:    sw     0(a4), a2
    00010258:    addi   a4, a4, 16
    0001025c:    bltu   a4, a3, 8172
    00010260:    bne    a2, zero, 8
    00010264:    ret
    00010268:    sub    a3, t1, a2
    0001026c:    slli   a3, a3, 2
    00010270:    auipc  t0, 0
    00010274:    add    a3, a3, t0
    00010278:    jalr   zero, 0(a3)
    0001027c:    sb     0(a4), a4
    00010280:    sb     0(a4), a3
    00010284:    sb     0(a4), a2
    00010288:    sb     0(a4), a1
    0001028c:    sb     0(a4), a0
    00010290:    sb     0(a4), s1
    00010294:    sb     0(a4), fp
    00010298:    sb     0(a4), t2
    0001029c:    sb     0(a4), t1
    000102a0:    sb     0(a4), t0
    000102a4:    sb     0(a4), tp
    000102a8:    sb     0(a4), gp
    000102ac:    sb     0(a4), sp
    000102b0:    sb     0(a4), ra
    000102b4:    sb     0(a4), zero
    000102b8:    ret
    000102bc:    andi   a1, a1, 255
    000102c0:    slli   a3, a1, 8
    000102c4:    or     a1, a1, a3
    000102c8:    slli   a3, a1, 16
    000102cc:    or     a1, a1, a3
    000102d0:    j      0
    000102d4:    slli   a3, a5, 2
    000102d8:    auipc  t0, 0
    000102dc:    add    a3, a3, t0
    000102e0:    addi   t0, ra, 0
    000102e4:    jalr   ra, 0(a3)
    000102e8:    addi   ra, t0, 0
    000102ec:    addi   a5, a5, 4080
    000102f0:    sub    a4, a4, a5
    000102f4:    add    a2, a2, a5
    000102f8:    bgeu   t1, a2, 8048
    000102fc:    j      0
__call_exitprocs:
    00010300:    addi   sp, sp, 4048
    00010304:    sw     0(sp), s8
    00010308:    lw     s4, 3112(gp)
    0001030c:    sw     0(sp), zero
    00010310:    sw     0(sp), a2
    00010314:    lw     s2, 328(s4)
    00010318:    sw     0(sp), fp
    0001031c:    sw     0(sp), tp
    00010320:    sw     0(sp), t3
    00010324:    sw     0(sp), s4
    00010328:    sw     0(sp), a6
    0001032c:    sw     0(sp), a2
    00010330:    sw     0(sp), fp
    00010334:    beq    s2, zero, 64
    00010338:    addi   s6, a0, 0
    0001033c:    addi   s7, a1, 0
    00010340:    li     s5, 1
    00010344:    li     s3, 4095
    00010348:    lw     s1, 4(s2)
    0001034c:    addi   fp, s1, 4095
    00010350:    blt    fp, zero, 36
    00010354:    slli   s1, s1, 2
    00010358:    add    s1, s2, s1
    0001035c:    beq    s7, zero, 72
    00010360:    lw     a5, 260(s1)
    00010364:    beq    a5, s7, 64
    00010368:    addi   fp, fp, 4095
    0001036c:    addi   s1, s1, 4092
    00010370:    bne    fp, s3, 8172
    00010374:    lw     ra, 44(sp)
    00010378:    lw     fp, 40(sp)
    0001037c:    lw     s1, 36(sp)
    00010380:    lw     s2, 32(sp)
    00010384:    lw     s3, 28(sp)
    00010388:    lw     s4, 24(sp)
    0001038c:    lw     s5, 20(sp)
    00010390:    lw     s6, 16(sp)
    00010394:    lw     s7, 12(sp)
    00010398:    lw     s8, 8(sp)
    0001039c:    addi   sp, sp, 48
    000103a0:    ret
    000103a4:    lw     a5, 4(s2)
    000103a8:    lw     a3, 4(s1)
    000103ac:    addi   a5, a5, 4095
    000103b0:    beq    a5, fp, 92
    000103b4:    sw     0(s1), tp
    000103b8:    beq    a3, zero, 8112
    000103bc:    lw     a5, 392(s2)
    000103c0:    sll    a4, s5, fp
    000103c4:    lw     s8, 4(s2)
    000103c8:    and    a5, a4, a5
    000103cc:    bne    a5, zero, 36
    000103d0:    jalr   ra, 0(a3)
    000103d4:    lw     a4, 4(s2)
    000103d8:    lw     a5, 328(s4)
    000103dc:    bne    a4, s8, 8
    000103e0:    beq    a5, s2, 8072
    000103e4:    beq    a5, zero, 8080
    000103e8:    addi   s2, a5, 0
    000103ec:    j      0
    000103f0:    lw     a5, 396(s2)
    000103f4:    lw     a1, 132(s1)
    000103f8:    and    a4, a4, a5
    000103fc:    bne    a4, zero, 24
    00010400:    addi   a0, s6, 0
    00010404:    jalr   ra, 0(a3)
    00010408:    j      0
    0001040c:    sw     0(s2), tp
    00010410:    j      0
    00010414:    addi   a0, a1, 0
    00010418:    jalr   ra, 0(a3)
    0001041c:    j      0
atexit:
    00010420:    addi   a1, a0, 0
    00010424:    li     a3, 0
    00010428:    li     a2, 0
    0001042c:    li     a0, 0
    00010430:    j      0
__libc_fini_array:
    00010434:    addi   sp, sp, 4080
    00010438:    sw     0(sp), fp
    0001043c:    lui    a5, 0x11000
    00010440:    lui    fp, 0x11000
    00010444:    addi   a5, a5, 1388
    00010448:    addi   fp, fp, 1392
    0001044c:    sub    fp, fp, a5
    00010450:    sw     0(sp), tp
    00010454:    sw     0(sp), a2
    00010458:    srai   s1, fp, 2
    0001045c:    beq    s1, zero, 32
    00010460:    addi   fp, fp, 4092
    00010464:    add    fp, fp, a5
    00010468:    lw     a5, 0(fp)
    0001046c:    addi   s1, s1, 4095
    00010470:    addi   fp, fp, 4092
    00010474:    jalr   ra, 0(a5)
    00010478:    bne    s1, zero, 8176
    0001047c:    lw     ra, 12(sp)
    00010480:    lw     fp, 8(sp)
    00010484:    lw     s1, 4(sp)
    00010488:    addi   sp, sp, 16
    0001048c:    ret
__register_exitproc:
    00010490:    lw     a4, 3112(gp)
    00010494:    lw     a5, 328(a4)
    00010498:    beq    a5, zero, 88
    0001049c:    lw     a4, 4(a5)
    000104a0:    li     a6, 31
    000104a4:    blt    a6, a4, 124
    000104a8:    slli   a6, a4, 2
    000104ac:    beq    a0, zero, 44
    000104b0:    add    t1, a5, a6
    000104b4:    sw     0(t1), fp
    000104b8:    lw     a7, 392(a5)
    000104bc:    li     a2, 1
    000104c0:    sll    a2, a2, a4
    000104c4:    or     a7, a7, a2
    000104c8:    sw     0(a5), fp
    000104cc:    sw     0(t1), fp
    000104d0:    li     a3, 2
    000104d4:    beq    a0, a3, 40
    000104d8:    addi   a4, a4, 1
    000104dc:    sw     0(a5), tp
    000104e0:    add    a5, a5, a6
    000104e4:    sw     0(a5), fp
    000104e8:    li     a0, 0
    000104ec:    ret
    000104f0:    addi   a5, a4, 332
    000104f4:    sw     0(a4), fp
    000104f8:    j      0
    000104fc:    lw     a3, 396(a5)
    00010500:    addi   a4, a4, 1
    00010504:    sw     0(a5), tp
    00010508:    or     a3, a3, a2
    0001050c:    sw     0(a5), a2
    00010510:    add    a5, a5, a6
    00010514:    sw     0(a5), fp
    00010518:    li     a0, 0
    0001051c:    ret
    00010520:    li     a0, 4095
    00010524:    ret
_exit:
    00010528:    li     a7, 93
    0001052c:    ecall
    00010530:    blt    a0, zero, 8
    00010534:    j      0
    00010538:    addi   sp, sp, 4080
    0001053c:    sw     0(sp), fp
    00010540:    addi   fp, a0, 0
    00010544:    sw     0(sp), a2
    00010548:    sub    fp, zero, fp
    0001054c:    jal    ra, 0
    00010550:    sw     0(a0), zero
    00010554:    j      0
__errno:
    00010558:    lw     a0, 3120(gp)
    0001055c:    ret
```