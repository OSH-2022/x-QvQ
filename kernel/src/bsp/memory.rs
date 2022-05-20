pub mod map {
    pub mod mmio {
        pub const _BASE: usize = 0x3f00_0000;
        pub mod auxiliary {
            pub const BASE: usize = 0xffff_ff80_0000_c000;      // FIXME: use symbol from linker
        }
    }
}
