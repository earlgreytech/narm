extern crate narm;
mod common;

use common::*;
use narm::narmvm::*;

/*

Integration test for Branch operators

Included varieties:

B <label> T2            - Branch unconditionally by label, jump size restricted to signed 11 bits
B<C> <label> T1         - Branch conditionally by label, jump size restricted to signed 8 bits
BX <Rm> T1              - Branch by register
BL <label> T1 (32-bit)  - Branch by label, set link register, jump size restricted to even signed 25 bits (?????)
BLX <Rm> T1             - Branch by register, set link register

General test cases:

- Branch forward
- Branch backward
- Branch with non-4-aligned op
- Branch far forward (Causing memory error)
- Branch far backward (Causing memory error)

Special test case for BLX <Rm> T1:

- Branch and then branch back using address saved in link register

Special test case for B<C> <label> T1:

- Test all different conditions

Special test case for BL <label> T1 (32-bit):

- Test proper function with different op alignment

The reference for these tests is currently official documentations and a QEMU-based VM
TODO: Test against a hardware Cortex-M0 to make sure it's actually up to spec?

*/

// String representation of ops for use in debug output
const OPCODES: &'static [&'static str] = &[
    "B <label> T2",
    "B<C> <label> T1",
    "BX <Rm> T1",
    "BL <label> T1 (32-bit)",
    "BLX <Rm> T1",
];

// Simple constant for number of opcodes tested in this file
const NUM_OPCODES: &'static usize = &5;

// Branch forward
#[test]
pub fn test_branch_forward() {
    println!("\n>>> Branch ops test case: Branch forward \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[1] = Some(code_mem_address(2 * OP_SIZE)));

    // VM initialization
    let post_ops = "
        svc             #0x01
        test1:
        movs r0,        #0xCD
        svc             #0xFF
        ";

    // 0: B <label> T2"
    let ops0 = format!("b test1 {}", post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 0, asm_var = ops0);

    // 1: B<C> <label> T1 (BNE -> if flag Z = 0)
    let ops1 = format!("bne test1 {}", post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 1, asm_var = ops1);

    // 2: BX <Rm> T1
    let ops2 = format!("bx r1 {}", post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 2, asm_var = ops2);

    // 3: BL <label> T1 (32-bit)
    let ops3 = format!("bl test1 {}", post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 3, asm_var = ops3);
    vm_states[3].r[14] = Some(code_mem_address(OP_SIZE_32BIT)); // Address to the op after the last BL instruction is loaded to link reg

    // 4: BLX <Rm> T1
    let ops4 = format!("blx r1 {}", post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 4, asm_var = ops4);
    vm_states[4].r[14] = Some(code_mem_address(OP_SIZE)); // Address to the op after the last BLX instruction is loaded to link reg

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xCD));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Branch backward
#[test]
pub fn test_branch_backward() {
    println!("\n>>> Branch ops test case: Branch backward \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[1] = Some(code_mem_address(0)));
    set_for_all!(vm_states[ops_to_test].pc_address = Some(code_mem_address(2 * OP_SIZE)));

    // VM initialization
    let pre_ops = "
        test1:
        movs r0,        #0xCD
        svc             #0xFF
        ";

    let post_ops = "
        movs r0,        #0x00
        svc             #0x01
        ";

    // 0: B <label> T2"
    let ops0 = format!("{}b test1 {}", pre_ops, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 0, asm_var = ops0);

    // 1: B<C> <label> T1 (BNE -> if flag Z = 0)
    let ops1 = format!("{}bne test1 {}", pre_ops, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 1, asm_var = ops1);

    // 2: BX <Rm> T1
    let ops2 = format!("{}bx r1 {}", pre_ops, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 2, asm_var = ops2);

    // 3: BL <label> T1 (32-bit)
    let ops3 = format!("{}bl test1 {}", pre_ops, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 3, asm_var = ops3);
    vm_states[3].r[14] = Some(code_mem_address(OP_SIZE_32BIT + OP_SIZE * 2)); // Address to the op after the last BL instruction is loaded to link reg

    // 4: BLX <Rm> T1
    let ops4 = format!("{}blx r1 {}", pre_ops, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 4, asm_var = ops4);
    vm_states[4].r[14] = Some(code_mem_address(3 * OP_SIZE)); // Address to the op after the last BLX instruction is loaded to link reg

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xCD));
    set_for_all!(vm_states[ops_to_test].pc_address = None);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Branch with non-4-aligned op
// This test is done because of a previous issue where non-4-aligned ops would jump to the wrong address
#[test]
pub fn test_branch_alignment() {
    println!("\n>>> Branch ops test case: Branch with non-4-aligned op \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![0, 1, 2, 3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[1] = Some(code_mem_address(5 * OP_SIZE)));

    // VM initialization
    // A leading NOP (2-byte no-op) is used to make tested op not-4-aligned
    let no_op = "
        nop
        ";
    let post_ops = "
        nop
        nop
        svc             #0x01
        test1:
        movs r0,        #0xCD
        svc             #0xFF
        ";

    // 0: B <label> T2"
    let ops0 = format!("{}b test1 {}", no_op, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 0, asm_var = ops0);

    // 1: B<C> <label> T1 (BNE -> if flag Z = 0)
    let ops1 = format!("{}bne test1 {}", no_op, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 1, asm_var = ops1);

    // 2: BX <Rm> T1
    let ops2 = format!("{}bx r1 {}", no_op, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 2, asm_var = ops2);

    // 3: BL <label> T1 (32-bit)
    let ops3 = format!("{}bl test1 {}", no_op, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 3, asm_var = ops3);
    vm_states[3].r[14] = Some(code_mem_address(OP_SIZE_32BIT + 1 * OP_SIZE)); // Address to the op after the last BL instruction is loaded to link reg

    // 4: BLX <Rm> T1
    let ops4 = format!("{}blx r1 {}", no_op, post_ops);
    create_vm!(arrays = (vms, vm_states), op_id = 4, asm_var = ops4);
    vm_states[4].r[14] = Some(code_mem_address(2 * OP_SIZE)); // Address to the op after the last BLX instruction is loaded to link reg

    // Common expected post-execution state
    set_for_all!(vm_states[ops_to_test].r[0] = Some(0xCD));

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Branch far forward (Causing memory error)
#[test]
pub fn test_branch_far_forward() {
    println!("\n>>> Branch ops test case: Branch far forward (Causing memory error) \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2, 3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[1] = Some(code_mem_address(100000 * OP_SIZE)));
    set_for_all!(vm_states[ops_to_test].expect_exec_error = true);

    // VM initialization

    // 0: B <label> T2" - Not applicable

    // 1: B<C> <label> T1  - Not applicable

    // 2: BX <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "bx r1"
    );

    // 3: BL <label> T1 (32-bit)
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "bl #0xF0001"
    );
    vm_states[3].r[14] = Some(code_mem_address(OP_SIZE_32BIT)); // Address to the op after the last BL instruction is loaded to link reg

    // 4: BLX <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "blx r1"
    );
    vm_states[4].r[14] = Some(code_mem_address(OP_SIZE)); // Address to the op after the last BLX instruction is loaded to link reg

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Branch far backward (Causing memory error)
#[test]
pub fn test_branch_far_backward() {
    println!("\n>>> Branch ops test case: Branch far backward (Causing memory error) \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![2, 3, 4];

    // Common pre-execution state
    set_for_all!(vm_states[ops_to_test].r[1] = Some(0x50));
    set_for_all!(vm_states[ops_to_test].expect_exec_error = true);

    // VM initialization

    // 0: B <label> T2" - Not applicable

    // 1: B<C> <label> T1  - Not applicable

    // 2: BX <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 2,
        asm_literal_add_svc = "bx r1"
    );

    // 3: BL <label> T1 (32-bit)
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal_add_svc = "bl #0x50"
    );
    vm_states[3].r[14] = Some(code_mem_address(OP_SIZE_32BIT)); // Address to the op after the last BL instruction is loaded to link reg

    // 4: BLX <Rm> T1
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal_add_svc = "blx r1"
    );
    vm_states[4].r[14] = Some(code_mem_address(OP_SIZE)); // Address to op after the last BLX instruction is loaded to link reg

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Branch and then branch back using address saved in link register
#[test]
pub fn test_branch_and_return() {
    println!("\n>>> BLX <Rm> T1 special test case: Branch and then branch back using address saved in link register \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![4];

    // 4: BLX <Rm> T1
    vm_states[4].r[1] = Some(code_mem_address(2 * OP_SIZE));
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 4,
        asm_literal = "
        blx r1
        svc             #0xFF
        movs r0,        #0xCD
        blx r14
        movs r0,        #0x01
        svc             #0x01
    "
    );
    vm_states[4].r[14] = Some(code_mem_address(4 * OP_SIZE)); // Address to op after the last BLX instruction is loaded to link reg
    vm_states[4].r[0] = Some(0xCD);

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}

// Test all different conditions
#[test]
pub fn test_branch_conditions() {
    println!("\n>>> B<C> <label> T1 special test case: Test all different conditions \n");

    // VM initialization
    let post_ops = "
        movs r0,        #0x01
        svc             #0x01
        test1:
        svc             #0xFF
        ";

    println!("\n>>> Running tests for 15 condition types \n");

    // EQ: Z == 1
    println!("\n>>> [1/14] Testing for condition type: EQ \n");
    let mut vm = create_vm_from_asm(&format!("beq test1 {}", post_ops));
    vm.cpsr.z = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // NE: Z == 0
    println!("\n>>> [2/14] Testing for condition type: NE \n");
    let mut vm = create_vm_from_asm(&format!("bne test1 {}", post_ops));
    vm.cpsr.z = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // CS: C == 1
    println!("\n>>> [3/14] Testing for condition type: CS \n");
    let mut vm = create_vm_from_asm(&format!("bcs test1 {}", post_ops));
    vm.cpsr.c = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // CC: C == 0
    println!("\n>>> [4/14] Testing for condition type: CC \n");
    let mut vm = create_vm_from_asm(&format!("bcc test1 {}", post_ops));
    vm.cpsr.c = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // MI: N == 1
    println!("\n>>> [5/14] Testing for condition type: MI \n");
    let mut vm = create_vm_from_asm(&format!("bmi test1 {}", post_ops));
    vm.cpsr.n = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // PL: N == 0
    println!("\n>>> [6/14] Testing for condition type: PL \n");
    let mut vm = create_vm_from_asm(&format!("bpl test1 {}", post_ops));
    vm.cpsr.n = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // VS: V == 1
    println!("\n>>> [7/14] Testing for condition type: VS \n");
    let mut vm = create_vm_from_asm(&format!("bvs test1 {}", post_ops));
    vm.cpsr.v = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // VC: V == 0
    println!("\n>>> [8/14] Testing for condition type: VC \n");
    let mut vm = create_vm_from_asm(&format!("bvc test1 {}", post_ops));
    vm.cpsr.v = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // HI: C == 1 AND Z == 0
    println!("\n>>> [9/14] Testing for condition type: HI \n");
    let mut vm = create_vm_from_asm(&format!("bhi test1 {}", post_ops));
    vm.cpsr.c = true;
    vm.cpsr.z = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // LS: C == 0 OR  Z == 1
    println!("\n>>> [10/14] Testing for condition type: LS \n");
    let mut vm = create_vm_from_asm(&format!("bls test1 {}", post_ops));
    vm.cpsr.c = false;
    vm.cpsr.z = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    let mut vm = create_vm_from_asm(&format!("bls test1 {}", post_ops));
    vm.cpsr.c = true;
    vm.cpsr.z = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // GE: N == V
    println!("\n>>> [11/14] Testing for condition type: GE \n");
    let mut vm = create_vm_from_asm(&format!("bge test1 {}", post_ops));
    vm.cpsr.n = true;
    vm.cpsr.v = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    let mut vm = create_vm_from_asm(&format!("bge test1 {}", post_ops));
    vm.cpsr.n = false;
    vm.cpsr.v = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // LT: N != V
    println!("\n>>> [12/14] Testing for condition type: LT \n");
    let mut vm = create_vm_from_asm(&format!("blt test1 {}", post_ops));
    vm.cpsr.n = true;
    vm.cpsr.v = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);
    let mut vm = create_vm_from_asm(&format!("blt test1 {}", post_ops));
    vm.cpsr.n = false;
    vm.cpsr.v = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // GT: Z == 0 AND N == V
    println!("\n>>> [13/14] Testing for condition type: GT \n");
    let mut vm = create_vm_from_asm(&format!("bgt test1 {}", post_ops));
    vm.cpsr.z = false;
    vm.cpsr.n = false;
    vm.cpsr.v = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    let mut vm = create_vm_from_asm(&format!("bgt test1 {}", post_ops));
    vm.cpsr.z = false;
    vm.cpsr.n = true;
    vm.cpsr.v = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    // LE: Z == 1 OR  N != V
    println!("\n>>> [14/14] Testing for condition type: LE \n");
    let mut vm = create_vm_from_asm(&format!("ble test1 {}", post_ops));
    vm.cpsr.z = true;
    vm.cpsr.n = true;
    vm.cpsr.v = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    let mut vm = create_vm_from_asm(&format!("ble test1 {}", post_ops));
    vm.cpsr.z = false;
    vm.cpsr.n = true;
    vm.cpsr.v = false;
    assert_eq!(vm.execute().unwrap(), 0xFF);

    let mut vm = create_vm_from_asm(&format!("ble test1 {}", post_ops));
    vm.cpsr.z = false;
    vm.cpsr.n = false;
    vm.cpsr.v = true;
    assert_eq!(vm.execute().unwrap(), 0xFF);
}

// Test proper function with different op alignment
// This is tested because BL (apparently?) doesn't follow standard for pc-relative address calculation unless op is aligned by 4 in memory
#[test]
pub fn test_branch_alignment_bl() {
    println!("\n>>> BL <label> T1 (32bit) special test case: Test proper function with different op alignment \n");

    // Arrays holding instances of VMs and matching state structs
    let mut vms: [NarmVM; *NUM_OPCODES] = Default::default();
    let mut vm_states: [VMState; *NUM_OPCODES] = Default::default();

    // Tell macros which op varieties are tested in this function
    let ops_to_test = vec![3];

    // 3: BL <label> T1 (32-bit)
    create_vm!(
        arrays = (vms, vm_states),
        op_id = 3,
        asm_literal = "
        bl label1
        svc             #0x01
        label1:
        bl label2
        svc             #0x02
        svc             #0x03
        label2:
        bl label3
        svc             #0x04
        svc             #0x05
        svc             #0x06
        label3:
        svc             #0xFF
        svc             #0x07
        svc             #0x08
    "
    );

    vm_states[3].r[14] = None; // Link registry is not important in this test

    run_test!(arrays = (vms, vm_states), op_ids = ops_to_test);
}
