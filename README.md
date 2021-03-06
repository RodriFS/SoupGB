# Soup

A gameboy emulator written in rust

# How to run

(Builds not available yet)

```
cargo run --release ./path/to/file.gb
```

# Keys

```
Move: Arrows
A: x key
B: z key
Select: Space
Start: Enter
```

# Status

- Audio not supported (yet)
- Some cartridges are not yet supported. See "Test status"

# Tests status:

## Blargg

| Test           | Status |
| -------------- | :----: |
| cpu_instr      |   👍   |
| instr_timing   |   👍   |
| halt_bug       |   👍   |
| interrupt_time |   ❌   |
| mem_timing     |   👍   |
| mem_timing-2   |   👍   |
| cgb_sound      |   ❌   |
| dmg_sound      |   ❌   |
| oam_bug        |   ❌   |

## Mooneye Acceptance tests

| Test                             | Status |
| -------------------------------- | :----: |
| add_sp_e_timing                  |   👍   |
| call_cc_timing                   |   👍   |
| call_cc_timing2                  |   👍   |
| call_timing                      |   👍   |
| call_timing2                     |   👍   |
| di_timing-GS                     |   ❌   |
| div_timing                       |   👍   |
| ei_sequence                      |   👍   |
| ei_timing                        |   👍   |
| halt_ime0_ei                     |   👍   |
| halt_ime0_nointr_timing          |   ❌   |
| halt_ime1_timing                 |   👍   |
| halt_ime1_timing2-GS             |   ❌   |
| if_ie_registers                  |   👍   |
| intr_timing                      |   👍   |
| jp_cc_timing                     |   👍   |
| jp_timing                        |   👍   |
| ld_hl_sp_e_timing                |   👍   |
| oam_dma_restart                  |   👍   |
| oam_dma_start                    |   ❌   |
| oam_dma_timing                   |   👍   |
| pop_timing                       |   👍   |
| push_timing                      |   👍   |
| rapid_di_ei                      |   👍   |
| ret_cc_timing                    |   👍   |
| ret_timing                       |   👍   |
| reti_intr_timing                 |   👍   |
| reti_timing                      |   👍   |
| rst_timing                       |   👍   |
| bits/mem_oam                     |   👍   |
| bits/reg_f                       |   👍   |
| bits/unused_hwio-GS              |   ❌   |
| instr/daa                        |   👍   |
| interrupts/ie_push               |   ❌   |
| oam_dma/basic                    |   👍   |
| oam_dma/reg_read                 |   👍   |
| oam_dma/sources-GS               |   ❌   |
| ppu/hblank_ly_scx_timing-GS      |   ❌   |
| ppu/intr_1_2_timing-GS           |   ❌   |
| ppu/intr_2_0_timing              |   ❌   |
| ppu/intr_2_mode0_timing          |   👍   |
| ppu/intr_2_mode0_timing_sprites  |   ❌   |
| ppu/intr_2_mode3_timing          |   👍   |
| ppu/intr_2_oam_ok_timing         |   👍   |
| ppu/lcdon_timing-GS              |   ❌   |
| ppu/lcdon_write_timing-GS        |   ❌   |
| ppu/stat_irq_blocking            |   ❌   |
| ppu/stat_lyc_onoff               |   ❌   |
| ppu/vblank_stat_intr-GS          |   ❌   |
| serial/boot_sclk_align-dmgABCmgb |   ❌   |
| timer/div_write                  |   👍   |
| timer/rapid_toggle               |   ❌   |
| timer/tim00                      |   👍   |
| timer/tim00_div_trigger          |   👍   |
| timer/tim01                      |   👍   |
| timer/tim01_div_trigger          |   👍   |
| timer/tim10                      |   👍   |
| timer/tim10_div_trigger          |   👍   |
| timer/tim11                      |   👍   |
| timer/tim11_div_trigger          |   👍   |
| timer/tima_reload                |   👍   |
| timer/tima_write_reloading       |   👍   |
| timer/tma_write_reloading        |   👍   |

## Mooneye Emulator Only tests

| Test                   | Status |
| ---------------------- | :----: |
| mbc1/bits_bank1        |   👍   |
| mbc1/bits_bank2        |   👍   |
| mbc1/bits_mode         |   👍   |
| mbc1/bits_ramg         |   👍   |
| mbc1/multicart_rom_8Mb |   ❌   |
| mbc1/ram_64kb          |   👍   |
| mbc1/ram_256kb         |   👍   |
| mbc1/rom_1Mb           |   👍   |
| mbc1/rom_2Mb           |   👍   |
| mbc1/rom_4Mb           |   👍   |
| mbc1/rom_8Mb           |   👍   |
| mbc1/rom_16Mb          |   👍   |
| mbc1/rom_512kb         |   👍   |

| Test             | Status |
| ---------------- | :----: |
| mbc2/bits_ramg   |   👍   |
| mbc2/bits_romb   |   👍   |
| mbc2/bits_unused |   👍   |
| mbc2/ram         |   👍   |
| mbc2/rom_1Mb     |   👍   |
| mbc2/rom_2Mb     |   👍   |
| mbc2/rom_512kb   |   👍   |

| Test |          Status          |
| ---- | :----------------------: |
| mbc3 | Supported but not tested |

# Disclaimer

This gameboy emulator was made for academic purposes only. If you're going to use it, please use uncopyrighted or open source games.
