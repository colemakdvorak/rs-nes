use super::Ppu;
use super::StatusRegister;

// TODO: Tests that require interaction with OAM registers have subtle non-obvious interactions.
// This doesn't make them suitable for a spec test, is there any way to make this nicer?

#[test]
fn memory_mapped_register_write_test() {
    let mut ppu = Ppu::new();

    // Writes to 0x2000 write the control register
    ppu.memory_mapped_register_write(0x2000, 0x1);
    assert_eq!(0x1, *ppu.ctrl_reg);

    // Writes to 0x2001 write the mask register
    ppu.memory_mapped_register_write(0x2001, 0x2);
    assert_eq!(0x2, *ppu.mask_reg);

    // Writes to 0x2003 write the oam addr register
    ppu.memory_mapped_register_write(0x2003, 0x3);
    assert_eq!(0x3, ppu.oam_address());

    // Writes to 0x2004 write the oam data register
    // We need to reset the oam address before writing and reading since it increments
    ppu.oam_set_address(0x0);
    ppu.memory_mapped_register_write(0x2004, 0x4);
    ppu.oam_set_address(0x0);
    assert_eq!(0x4, ppu.oam_read_data());

    // Writes to 0x2005 write the scroll register
    ppu.memory_mapped_register_write(0x2005, 0x5);
    assert_eq!(0x5, ppu.scroll);

    // Writes to 0x2006 write the vram addr register
    ppu.memory_mapped_register_write(0x2006, 0x6);
    assert_eq!(0x6, ppu.vram_addr);

    // Writes to 0x2007 write the vram data register
    ppu.memory_mapped_register_write(0x2007, 0x7);
    assert_eq!(0x7, ppu.vram_data);

    // Test mirroring: 0x2000-0x2007 are mirrored every 8 bytes to 0x3fff

    ppu.memory_mapped_register_write(0x2008, 0x8);
    assert_eq!(0x8, *ppu.ctrl_reg);

    ppu.memory_mapped_register_write(0x2009, 0x9);
    assert_eq!(0x9, *ppu.mask_reg);

    ppu.memory_mapped_register_write(0x200b, 0xa);
    assert_eq!(0xa, ppu.oam_address());

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam_set_address(0x0);
    ppu.memory_mapped_register_write(0x200c, 0xb);
    ppu.oam_set_address(0x0);
    assert_eq!(0xb, ppu.oam_read_data());

    ppu.memory_mapped_register_write(0x200d, 0xc);
    assert_eq!(0xc, ppu.scroll);

    ppu.memory_mapped_register_write(0x200e, 0xd);
    assert_eq!(0xd, ppu.vram_addr);

    ppu.memory_mapped_register_write(0x200f, 0xe);
    assert_eq!(0xe, ppu.vram_data);

    // Test mirroring on the tail end of the valid address space

    ppu.memory_mapped_register_write(0x3ff8, 0xf);
    assert_eq!(0xf, *ppu.ctrl_reg);

    ppu.memory_mapped_register_write(0x3ff9, 0x10);
    assert_eq!(0x10, *ppu.mask_reg);

    ppu.memory_mapped_register_write(0x3ffb, 0x11);
    assert_eq!(0x11, ppu.oam_address());

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam_set_address(0x0);
    ppu.memory_mapped_register_write(0x3ffc, 0x12);
    ppu.oam_set_address(0x0);
    assert_eq!(0x12, ppu.oam_read_data());

    ppu.memory_mapped_register_write(0x3ffd, 0x13);
    assert_eq!(0x13, ppu.scroll);

    ppu.memory_mapped_register_write(0x3ffe, 0x14);
    assert_eq!(0x14, ppu.vram_addr);

    ppu.memory_mapped_register_write(0x3fff, 0x15);
    assert_eq!(0x15, ppu.vram_data);
}

#[test]
fn memory_mapped_register_read_test() {
    let mut ppu = Ppu::new();

    ppu.ctrl_reg.set(0xf0);
    assert_eq!(0xf0, ppu.memory_mapped_register_read(0x2000));

    ppu.mask_reg.set(0xf1);
    assert_eq!(0xf1, ppu.memory_mapped_register_read(0x2001));

    ppu.status_reg = StatusRegister::new(0xf2);
    assert_eq!(0xf2, ppu.memory_mapped_register_read(0x2002));

    ppu.oam_write_data(0xf3);
    assert_eq!(0, ppu.memory_mapped_register_read(0x2003)); // write-only, should always read 0

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam_set_address(0x0);
    ppu.oam_write_data(0xf4);
    ppu.oam_set_address(0x0);
    assert_eq!(0xf4, ppu.memory_mapped_register_read(0x2004));

    ppu.scroll = 0xf5;
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x2005)); // write-only, should always read 0

    ppu.vram_addr = 0xf6;
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x2006)); // write-only, should always read 0

    ppu.vram_data = 0xf7;
    assert_eq!(0xf7, ppu.memory_mapped_register_read(0x2007));

    // Test mirroring: 0x2000-0x2007 are mirrored every 8 bytes to 0x3fff

    ppu.ctrl_reg.set(0xe0);
    assert_eq!(0xe0, ppu.memory_mapped_register_read(0x2008));

    ppu.mask_reg.set(0xe1);
    assert_eq!(0xe1, ppu.memory_mapped_register_read(0x2009));

    ppu.status_reg = StatusRegister::new(0xe2);
    assert_eq!(0xe2, ppu.memory_mapped_register_read(0x200a));

    ppu.oam_set_address(0xe3);
    assert_eq!(0, ppu.memory_mapped_register_read(0x200b)); // write-only, should always read 0

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam_set_address(0x0);
    ppu.oam_write_data(0xe4);
    ppu.oam_set_address(0x0);
    assert_eq!(0xe4, ppu.memory_mapped_register_read(0x200c));

    ppu.scroll = 0xe5;
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x200d)); // write-only, should always read 0

    ppu.vram_addr = 0xe6;
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x200e)); // write-only, should always read 0

    ppu.vram_data = 0xe7;
    assert_eq!(0xe7, ppu.memory_mapped_register_read(0x200f));

    // Test mirroring on the tail end of the valid address space

    ppu.ctrl_reg.set(0xd0);
    assert_eq!(0xd0, ppu.memory_mapped_register_read(0x3ff8));

    ppu.mask_reg.set(0xd1);
    assert_eq!(0xd1, ppu.memory_mapped_register_read(0x3ff9));

    ppu.status_reg = StatusRegister::new(0xd2);
    assert_eq!(0xd2, ppu.memory_mapped_register_read(0x3ffa));

    ppu.oam_set_address(0xd3);
    assert_eq!(0, ppu.memory_mapped_register_read(0x3ffb)); // write-only, should always read 0

    // We need to reset the oam address before writing and reading since it increments
    ppu.oam_set_address(0x0);
    ppu.oam_write_data(0xd4);
    ppu.oam_set_address(0x0);
    assert_eq!(0xd4, ppu.memory_mapped_register_read(0x3ffc));

    ppu.scroll = 0xd5;
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x3ffd)); // write-only, should always read 0

    ppu.vram_addr = 0xd6;
    assert_eq!(0x0, ppu.memory_mapped_register_read(0x3ffe)); // write-only, should always read 0

    ppu.vram_data = 0xd7;
    assert_eq!(0xd7, ppu.memory_mapped_register_read(0x3fff));
}