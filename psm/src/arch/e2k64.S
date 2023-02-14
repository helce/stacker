#include "psm.h"

.text
.globl rust_psm_stack_direction
.type rust_psm_stack_direction, @function
.align 8
rust_psm_stack_direction:
/* extern "C" fn() -> u8 */
{
    nop 5
    setwd wsz = 0x4, nfx = 0x1
    return %ctpr3
    addd,3 STACK_DIRECTION_DESCENDING, 0x0, %r0
}
{
    ct %ctpr3
}
.rust_psm_stack_direction_end:
.size rust_psm_stack_direction,.rust_psm_stack_direction_end-rust_psm_stack_direction


.globl rust_psm_stack_pointer
.type rust_psm_stack_pointer, @function
.align 8
rust_psm_stack_pointer:
/* extern "C" fn() -> *mut u8 */
{
    nop 5
    setwd wsz = 0x4, nfx = 0x1
    return %ctpr3
    getsp,0 0x0, %r0
}
{
    ct %ctpr3
}
.rust_psm_stack_pointer_end:
.size rust_psm_stack_pointer,.rust_psm_stack_pointer_end-rust_psm_stack_pointer
