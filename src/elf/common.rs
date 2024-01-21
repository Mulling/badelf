// Most of the non-sense here was stolen from `binutils/readelf.c` and `include/elf/common.h`

#[allow(non_snake_case)]
pub const NIDENT: usize = 0x10;

#[rustfmt::skip]
#[allow(dead_code)]
pub mod ei {
    use std::ops::Range;

    pub const        MAG: Range<usize> = 0x00..0x04;
    pub const      CLASS:        usize = 0x04;
    pub const       DATA:        usize = 0x05;
    pub const    VERSION:        usize = 0x06;
    pub const      OSABI:        usize = 0x07;
    pub const ABIVERSION:        usize = 0x08;
    pub const        PAD: Range<usize> = 0x09..0x10;

    // #[show(to_str(u8))]
    pub mod data {
        // #[as("little-endian")]
        const LITTLE: usize = 0x01;
        // #[as("big-endian")]
        const    BIG: usize = 0x02;
    }

    pub mod osabi {
        const     NONE: usize = 0x00; /* UNIX System V ABI */
        const     HPUX: usize = 0x01; /* HP-UX operating system */
        const   NETBSD: usize = 0x02; /* NetBSD */
        const      GNU: usize = 0x03; /* GNU */
        const    LINUX: usize = 0x03; /* Alias for ELFOSABI_GNU */
        const  SOLARIS: usize = 0x06; /* Solaris */
        const      AIX: usize = 0x07; /* AIX */
        const     IRIX: usize = 0x08; /* IRIX */
        const  FREEBSD: usize = 0x09; /* FreeBSD */
        const    TRU64: usize = 0x0A; /* TRU64 UNIX */
        const  MODESTO: usize = 0x0B; /* Novell Modesto */
        const  OPENBSD: usize = 0x0C; /* OpenBSD */
        const  OPENVMS: usize = 0x0D; /* OpenVMS */
        const      NSK: usize = 0x0E; /* Hewlett-Packard Non-Stop Kernel */
        const     AROS: usize = 0x0F; /* AROS */
        const  FENIXOS: usize = 0x10; /* FenixOS */
        const CLOUDABI: usize = 0x11; /* Nuxi CloudABI */
        const  OPENVOS: usize = 0x12; /* Stratus Technologies OpenVOS */
    }
}

pub fn ei_data_name(data: u8) -> &'static str {
    match data {
        0x01 => "little-endian",
        0x02 => "big-endian",
        _ => unreachable!(),
    }
}

pub fn ei_abi_name(abi: u8) -> &'static str {
    match abi {
        0x00 => "UNIX System V",
        0x01 => "HP-UX",
        0x02 => "NetBSD",
        0x03 => "GNU/Linux",
        0x04 => "GNU/Linux",
        0x05 => "Solaris",
        0x06 => "AIX",
        0x07 => "IRIX",
        0x08 => "FreeBSD",
        0x09 => "TRU64 UNIX",
        0x0A => "Novell Modesto",
        0x0B => "OpenBSD",
        0x0C => "OpenVMS",
        0x0D => "HP NonStop Kernel",
        0x0E => "AROS",
        0x0F => "FenixOS",
        0x10 => "Nuxi CloudABI",
        0x11 => "Stratus Technologies OpenVOS",
        _ => todo!("see: binutils/readelf.c get_osabi_name"),
    }
}

#[rustfmt::skip]
#[allow(dead_code)]
pub mod e {
    pub const    TYPE: usize = 0x10;
    pub const MACHINE: usize = 0x12;
    pub const VERSION: usize = 0x14;

    pub mod r#type {
        use std::ops::Range;

        pub const NONE:        usize = 0x00;
        pub const  REL:        usize = 0x04;
        pub const EXEC:        usize = 0x05;
        pub const  DYN:        usize = 0x06;
        pub const CORE:        usize = 0x07;
        pub const   OS: Range<usize> = 0xFE00..0xFF00;
        pub const PROC: Range<usize> = 0xFF00..0xFFFF;
    }

    pub mod machine {

    }
}

pub fn type_name(r#type: u16) -> &'static str {
    match r#type {
        0x0000 => "Unknown",
        0x0001 => "Relocatable file",
        0x0002 => "Executable file",
        0x0003 => "Shared object",
        0x0004 => "Core file",
        0xFE00..=0xFEFF => "Reserved inclusive range. Operating system specific.",
        0xFF00..=0xFFFF => "Reserved inclusive range. Processor specific.",
        _ => unreachable!(),
    }
}

pub fn machine_name(isa: u16) -> &'static str {
    match isa {
        0x0000 => "No specific instruction set",
        0x0001 => "AT&T WE 32100",
        0x0002 => "SPARC",
        0x0003 => "x86",
        0x0004 => "Motorola 68000 (M68k)",
        0x0005 => "Motorola 88000 (M88k)",
        0x0006 => "Intel MCU",
        0x0007 => "Intel 80860",
        0x0008 => "MIPS",
        0x0009 => "IBM System/370",
        0x000A => "MIPS RS3000 Little-endian",
        0x000F => "Hewlett-Packard PA-RISC",
        0x0013 => "Intel 80960",
        0x0014 => "PowerPC",
        0x0015 => "PowerPC (64-bit)",
        0x0016 => "S390, including S390x",
        0x0017 => "IBM SPU/SPC",
        0x0024 => "NEC V800",
        0x0025 => "Fujitsu FR20",
        0x0026 => "TRW RH-32",
        0x0027 => "Motorola RCE",
        0x0028 => "Arm (up to Armv7/AArch32)",
        0x0029 => "Digital Alpha",
        0x002A => "SuperH",
        0x002B => "SPARC Version 9",
        0x002C => "Siemens TriCore embedded processor",
        0x002D => "Argonaut RISC Core",
        0x002E => "Hitachi H8/300",
        0x002F => "Hitachi H8/300H",
        0x0030 => "Hitachi H8S",
        0x0031 => "Hitachi H8/500",
        0x0032 => "IA-64",
        0x0033 => "Stanford MIPS-X",
        0x0034 => "Motorola ColdFire",
        0x0035 => "Motorola M68HC12",
        0x0036 => "Fujitsu MMA Multimedia Accelerator",
        0x0037 => "Siemens PCP",
        0x0038 => "Sony nCPU embedded RISC processor",
        0x0039 => "Denso NDR1 microprocessor",
        0x003A => "Motorola Star*Core processor",
        0x003B => "Toyota ME16 processor",
        0x003C => "STMicroelectronics ST100 processor",
        0x003D => "Advanced Logic Corp. TinyJ embedded processor family",
        0x003E => "AMD x86-64",
        0x003F => "Sony DSP Processor",
        0x0040 => "Digital Equipment Corp. PDP-10",
        0x0041 => "Digital Equipment Corp. PDP-11",
        0x0042 => "Siemens FX66 microcontroller",
        0x0043 => "STMicroelectronics ST9+ 8/16 bit microcontroller",
        0x0044 => "STMicroelectronics ST7 8-bit microcontroller",
        0x0045 => "Motorola MC68HC16 Microcontroller",
        0x0046 => "Motorola MC68HC11 Microcontroller",
        0x0047 => "Motorola MC68HC08 Microcontroller",
        0x0048 => "Motorola MC68HC05 Microcontroller",
        0x0049 => "Silicon Graphics SVx",
        0x004A => "STMicroelectronics ST19 8-bit microcontroller",
        0x004B => "Digital VAX",
        0x004C => "Axis Communications 32-bit embedded processor",
        0x004D => "Infineon Technologies 32-bit embedded processor",
        0x004E => "Element 14 64-bit DSP Processor",
        0x004F => "LSI Logic 16-bit DSP Processor",
        0x008C => "TMS320C6000 Family",
        0x00AF => "MCST Elbrus e2k",
        0x00B7 => "Arm 64-bits (Armv8/AArch64)",
        0x00DC => "Zilog Z80",
        0x00F3 => "RISC-V",
        0x00F7 => "Berkeley Packet Filter",
        0x0101 => "WDC 65C816",
        _ => unreachable!(),
        // TODO:
        // #define EM_NONE		  0	/* No machine */
        // #define EM_M32		  1	/* AT&T WE 32100 */
        // #define EM_SPARC	  2	/* SUN SPARC */
        // #define EM_386		  3	/* Intel 80386 */
        // #define EM_68K		  4	/* Motorola m68k family */
        // #define EM_88K		  5	/* Motorola m88k family */
        // #define EM_IAMCU	  6	/* Intel MCU */
        // #define EM_860		  7	/* Intel 80860 */
        // #define EM_MIPS		  8	/* MIPS R3000 (officially, big-endian only) */
        // #define EM_S370		  9	/* IBM System/370 */
        // #define EM_MIPS_RS3_LE	 10	/* MIPS R3000 little-endian (Oct 4 1999 Draft).  Deprecated.  */
        // #define EM_OLD_SPARCV9	 11	/* Old version of Sparc v9, from before the ABI.  Deprecated.  */
        // #define EM_res011	 11	/* Reserved */
        // #define EM_res012	 12	/* Reserved */
        // #define EM_res013	 13	/* Reserved */
        // #define EM_res014	 14	/* Reserved */
        // #define EM_PARISC	 15	/* HPPA */
        // #define EM_res016	 16	/* Reserved */
        // #define EM_PPC_OLD	 17	/* Old version of PowerPC.  Deprecated.  */
        // #define EM_VPP550	 17	/* Fujitsu VPP500 */
        // #define EM_SPARC32PLUS	 18	/* Sun's "v8plus" */
        // #define EM_960		 19	/* Intel 80960 */
        // #define EM_PPC		 20	/* PowerPC */
        // #define EM_PPC64	 21	/* 64-bit PowerPC */
        // #define EM_S390		 22	/* IBM S/390 */
        // #define EM_SPU		 23	/* Sony/Toshiba/IBM SPU */
        // #define EM_res024	 24	/* Reserved */
        // #define EM_res025	 25	/* Reserved */
        // #define EM_res026	 26	/* Reserved */
        // #define EM_res027	 27	/* Reserved */
        // #define EM_res028	 28	/* Reserved */
        // #define EM_res029	 29	/* Reserved */
        // #define EM_res030	 30	/* Reserved */
        // #define EM_res031	 31	/* Reserved */
        // #define EM_res032	 32	/* Reserved */
        // #define EM_res033	 33	/* Reserved */
        // #define EM_res034	 34	/* Reserved */
        // #define EM_res035	 35	/* Reserved */
        // #define EM_V800		 36	/* NEC V800 series */
        // #define EM_FR20		 37	/* Fujitsu FR20 */
        // #define EM_RH32		 38	/* TRW RH32 */
        // #define EM_MCORE	 39	/* Motorola M*Core */ /* May also be taken by Fujitsu MMA */
        // #define EM_RCE		 39	/* Old name for MCore */
        // #define EM_ARM		 40	/* ARM */
        // #define EM_OLD_ALPHA	 41	/* Digital Alpha */
        // #define EM_SH		 42	/* Renesas (formerly Hitachi) / SuperH SH */
        // #define EM_SPARCV9	 43	/* SPARC v9 64-bit */
        // #define EM_TRICORE	 44	/* Siemens Tricore embedded processor */
        // #define EM_ARC		 45	/* ARC Cores */
        // #define EM_H8_300	 46	/* Renesas (formerly Hitachi) H8/300 */
        // #define EM_H8_300H	 47	/* Renesas (formerly Hitachi) H8/300H */
        // #define EM_H8S		 48	/* Renesas (formerly Hitachi) H8S */
        // #define EM_H8_500	 49	/* Renesas (formerly Hitachi) H8/500 */
        // #define EM_IA_64	 50	/* Intel IA-64 Processor */
        // #define EM_MIPS_X	 51	/* Stanford MIPS-X */
        // #define EM_COLDFIRE	 52	/* Motorola Coldfire */
        // #define EM_68HC12	 53	/* Motorola M68HC12 */
        // #define EM_MMA		 54	/* Fujitsu Multimedia Accelerator */
        // #define EM_PCP		 55	/* Siemens PCP */
        // #define EM_NCPU		 56	/* Sony nCPU embedded RISC processor */
        // #define EM_NDR1		 57	/* Denso NDR1 microprocessor */
        // #define EM_STARCORE	 58	/* Motorola Star*Core processor */
        // #define EM_ME16		 59	/* Toyota ME16 processor */
        // #define EM_ST100	 60	/* STMicroelectronics ST100 processor */
        // #define EM_TINYJ	 61	/* Advanced Logic Corp. TinyJ embedded processor */
        // #define EM_X86_64	 62	/* Advanced Micro Devices X86-64 processor */
        // #define EM_PDSP		 63	/* Sony DSP Processor */
        // #define EM_PDP10	 64	/* Digital Equipment Corp. PDP-10 */
        // #define EM_PDP11	 65	/* Digital Equipment Corp. PDP-11 */
        // #define EM_FX66		 66	/* Siemens FX66 microcontroller */
        // #define EM_ST9PLUS	 67	/* STMicroelectronics ST9+ 8/16 bit microcontroller */
        // #define EM_ST7		 68	/* STMicroelectronics ST7 8-bit microcontroller */
        // #define EM_68HC16	 69	/* Motorola MC68HC16 Microcontroller */
        // #define EM_68HC11	 70	/* Motorola MC68HC11 Microcontroller */
        // #define EM_68HC08	 71	/* Motorola MC68HC08 Microcontroller */
        // #define EM_68HC05	 72	/* Motorola MC68HC05 Microcontroller */
        // #define EM_SVX		 73	/* Silicon Graphics SVx */
        // #define EM_ST19		 74	/* STMicroelectronics ST19 8-bit cpu */
        // #define EM_VAX		 75	/* Digital VAX */
        // #define EM_CRIS		 76	/* Axis Communications 32-bit embedded processor */
        // #define EM_JAVELIN	 77	/* Infineon Technologies 32-bit embedded cpu */
        // #define EM_FIREPATH	 78	/* Element 14 64-bit DSP processor */
        // #define EM_ZSP		 79	/* LSI Logic's 16-bit DSP processor */
        // #define EM_MMIX		 80	/* Donald Knuth's educational 64-bit processor */
        // #define EM_HUANY	 81	/* Harvard's machine-independent format */
        // #define EM_PRISM	 82	/* SiTera Prism */
        // #define EM_AVR		 83	/* Atmel AVR 8-bit microcontroller */
        // #define EM_FR30		 84	/* Fujitsu FR30 */
        // #define EM_D10V		 85	/* Mitsubishi D10V */
        // #define EM_D30V		 86	/* Mitsubishi D30V */
        // #define EM_V850		 87	/* Renesas V850 (formerly NEC V850) */
        // #define EM_M32R		 88	/* Renesas M32R (formerly Mitsubishi M32R) */
        // #define EM_MN10300	 89	/* Matsushita MN10300 */
        // #define EM_MN10200	 90	/* Matsushita MN10200 */
        // #define EM_PJ		 91	/* picoJava */
        // #define EM_OR1K		 92	/* OpenRISC 1000 32-bit embedded processor */
        // #define EM_ARC_COMPACT	 93	/* ARC International ARCompact processor */
        // #define EM_XTENSA	 94	/* Tensilica Xtensa Architecture */
        // #define EM_SCORE_OLD	 95	/* Old Sunplus S+core7 backend magic number. Written in the absence of an ABI.  */
        // #define EM_VIDEOCORE	 95	/* Alphamosaic VideoCore processor */
        // #define EM_TMM_GPP	 96	/* Thompson Multimedia General Purpose Processor */
        // #define EM_NS32K	 97	/* National Semiconductor 32000 series */
        // #define EM_TPC		 98	/* Tenor Network TPC processor */
        // #define EM_PJ_OLD	 99	/* Old value for picoJava.  Deprecated.  */
        // #define EM_SNP1K	 99	/* Trebia SNP 1000 processor */
        // #define EM_ST200	100	/* STMicroelectronics ST200 microcontroller */
        // #define EM_IP2K		101	/* Ubicom IP2022 micro controller */
        // #define EM_MAX		102	/* MAX Processor */
        // #define EM_CR		103	/* National Semiconductor CompactRISC */
        // #define EM_F2MC16	104	/* Fujitsu F2MC16 */
        // #define EM_MSP430	105	/* TI msp430 micro controller */
        // #define EM_BLACKFIN	106	/* ADI Blackfin */
        // #define EM_SE_C33	107	/* S1C33 Family of Seiko Epson processors */
        // #define EM_SEP		108	/* Sharp embedded microprocessor */
        // #define EM_ARCA		109	/* Arca RISC Microprocessor */
        // #define EM_UNICORE	110	/* Microprocessor series from PKU-Unity Ltd. and MPRC of Peking University */
        // #define EM_EXCESS	111	/* eXcess: 16/32/64-bit configurable embedded CPU */
        // #define EM_DXP		112	/* Icera Semiconductor Inc. Deep Execution Processor */
        // #define EM_ALTERA_NIOS2	113	/* Altera Nios II soft-core processor */
        // #define EM_CRX		114	/* National Semiconductor CRX */
        // #define EM_CR16_OLD	115	/* Old, value for National Semiconductor CompactRISC.  Deprecated.  */
        // #define EM_XGATE	115	/* Motorola XGATE embedded processor */
        // #define EM_C166		116	/* Infineon C16x/XC16x processor */
        // #define EM_M16C		117	/* Renesas M16C series microprocessors */
        // #define EM_DSPIC30F	118	/* Microchip Technology dsPIC30F Digital Signal Controller */
        // #define EM_CE		119	/* Freescale Communication Engine RISC core */
        // #define EM_M32C		120	/* Renesas M32C series microprocessors */
        // #define EM_res121	121	/* Reserved */
        // #define EM_res122	122	/* Reserved */
        // #define EM_res123	123	/* Reserved */
        // #define EM_res124	124	/* Reserved */
        // #define EM_res125	125	/* Reserved */
        // #define EM_res126	126	/* Reserved */
        // #define EM_res127	127	/* Reserved */
        // #define EM_res128	128	/* Reserved */
        // #define EM_res129	129	/* Reserved */
        // #define EM_res130	130	/* Reserved */
        // #define EM_TSK3000	131	/* Altium TSK3000 core */
        // #define EM_RS08		132	/* Freescale RS08 embedded processor */
        // #define EM_res133	133	/* Reserved */
        // #define EM_ECOG2	134	/* Cyan Technology eCOG2 microprocessor */
        // #define EM_SCORE	135	/* Sunplus Score */
        // #define EM_SCORE7	135	/* Sunplus S+core7 RISC processor */
        // #define EM_DSP24	136	/* New Japan Radio (NJR) 24-bit DSP Processor */
        // #define EM_VIDEOCORE3	137	/* Broadcom VideoCore III processor */
        // #define EM_LATTICEMICO32 138	/* RISC processor for Lattice FPGA architecture */
        // #define EM_SE_C17	139	/* Seiko Epson C17 family */
        // #define EM_TI_C6000	140	/* Texas Instruments TMS320C6000 DSP family */
        // #define EM_TI_C2000	141	/* Texas Instruments TMS320C2000 DSP family */
        // #define EM_TI_C5500	142	/* Texas Instruments TMS320C55x DSP family */
        // #define EM_res143	143	/* Reserved */
        // #define EM_TI_PRU	144	/* Texas Instruments Programmable Realtime Unit */
        // #define EM_res145	145	/* Reserved */
        // #define EM_res146	146	/* Reserved */
        // #define EM_res147	147	/* Reserved */
        // #define EM_res148	148	/* Reserved */
        // #define EM_res149	149	/* Reserved */
        // #define EM_res150	150	/* Reserved */
        // #define EM_res151	151	/* Reserved */
        // #define EM_res152	152	/* Reserved */
        // #define EM_res153	153	/* Reserved */
        // #define EM_res154	154	/* Reserved */
        // #define EM_res155	155	/* Reserved */
        // #define EM_res156	156	/* Reserved */
        // #define EM_res157	157	/* Reserved */
        // #define EM_res158	158	/* Reserved */
        // #define EM_res159	159	/* Reserved */
        // #define EM_MMDSP_PLUS	160	/* STMicroelectronics 64bit VLIW Data Signal Processor */
        // #define EM_CYPRESS_M8C	161	/* Cypress M8C microprocessor */
        // #define EM_R32C		162	/* Renesas R32C series microprocessors */
        // #define EM_TRIMEDIA	163	/* NXP Semiconductors TriMedia architecture family */
        // #define EM_QDSP6	164	/* QUALCOMM DSP6 Processor */
        // #define EM_8051		165	/* Intel 8051 and variants */
        // #define EM_STXP7X	166	/* STMicroelectronics STxP7x family */
        // #define EM_NDS32	167	/* Andes Technology compact code size embedded RISC processor family */
        // #define EM_ECOG1	168	/* Cyan Technology eCOG1X family */
        // #define EM_ECOG1X	168	/* Cyan Technology eCOG1X family */
        // #define EM_MAXQ30	169	/* Dallas Semiconductor MAXQ30 Core Micro-controllers */
        // #define EM_XIMO16	170	/* New Japan Radio (NJR) 16-bit DSP Processor */
        // #define EM_MANIK	171	/* M2000 Reconfigurable RISC Microprocessor */
        // #define EM_CRAYNV2	172	/* Cray Inc. NV2 vector architecture */
        // #define EM_RX		173	/* Renesas RX family */
        // #define EM_METAG	174	/* Imagination Technologies Meta processor architecture */
        // #define EM_MCST_ELBRUS	175	/* MCST Elbrus general purpose hardware architecture */
        // #define EM_ECOG16	176	/* Cyan Technology eCOG16 family */
        // #define EM_CR16		177	/* National Semiconductor CompactRISC 16-bit processor */
        // #define EM_ETPU		178	/* Freescale Extended Time Processing Unit */
        // #define EM_SLE9X	179	/* Infineon Technologies SLE9X core */
        // #define EM_L1OM		180	/* Intel L1OM */
        // #define EM_K1OM		181	/* Intel K1OM */
        // #define EM_INTEL182	182	/* Reserved by Intel */
        // #define EM_AARCH64	183	/* ARM 64-bit architecture */
        // #define EM_ARM184	184	/* Reserved by ARM */
        // #define EM_AVR32	185	/* Atmel Corporation 32-bit microprocessor family */
        // #define EM_STM8		186	/* STMicroeletronics STM8 8-bit microcontroller */
        // #define EM_TILE64	187	/* Tilera TILE64 multicore architecture family */
        // #define EM_TILEPRO	188	/* Tilera TILEPro multicore architecture family */
        // #define EM_MICROBLAZE	189	/* Xilinx MicroBlaze 32-bit RISC soft processor core */
        // #define EM_CUDA		190	/* NVIDIA CUDA architecture */
        // #define EM_TILEGX	191	/* Tilera TILE-Gx multicore architecture family */
        // #define EM_CLOUDSHIELD 	192 	/* CloudShield architecture family */
        // #define EM_COREA_1ST 	193 	/* KIPO-KAIST Core-A 1st generation processor family */
        // #define EM_COREA_2ND 	194 	/* KIPO-KAIST Core-A 2nd generation processor family */
        // #define EM_ARC_COMPACT2 195	/* Synopsys ARCompact V2 */
        // #define EM_OPEN8 	196 	/* Open8 8-bit RISC soft processor core */
        // #define EM_RL78		197	/* Renesas RL78 family.  */
        // #define EM_VIDEOCORE5 	198 	/* Broadcom VideoCore V processor */
        // #define EM_78K0R	199	/* Renesas 78K0R.  */
        // #define EM_56800EX 	200 	/* Freescale 56800EX Digital Signal Controller (DSC) */
        // #define EM_BA1 		201 	/* Beyond BA1 CPU architecture */
        // #define EM_BA2 		202 	/* Beyond BA2 CPU architecture */
        // #define EM_XCORE 	203 	/* XMOS xCORE processor family */
        // #define EM_MCHP_PIC 	204 	/* Microchip 8-bit PIC(r) family */
        // #define EM_INTELGT	205	/* Intel Graphics Technology */
        // #define EM_INTEL206	206	/* Reserved by Intel */
        // #define EM_INTEL207	207	/* Reserved by Intel */
        // #define EM_INTEL208	208	/* Reserved by Intel */
        // #define EM_INTEL209	209	/* Reserved by Intel */
        // #define EM_KM32 	210 	/* KM211 KM32 32-bit processor */
        // #define EM_KMX32 	211 	/* KM211 KMX32 32-bit processor */
        // #define EM_KMX16 	212 	/* KM211 KMX16 16-bit processor */
        // #define EM_KMX8 	213 	/* KM211 KMX8 8-bit processor */
        // #define EM_KVARC 	214 	/* KM211 KVARC processor */
        // #define EM_CDP 		215 	/* Paneve CDP architecture family */
        // #define EM_COGE 	216 	/* Cognitive Smart Memory Processor */
        // #define EM_COOL 	217 	/* Bluechip Systems CoolEngine */
        // #define EM_NORC 	218 	/* Nanoradio Optimized RISC */
        // #define EM_CSR_KALIMBA 	219 	/* CSR Kalimba architecture family */
        // #define EM_Z80 		220 	/* Zilog Z80 */
        // #define EM_VISIUM	221	/* Controls and Data Services VISIUMcore processor */
        // #define EM_FT32         222     /* FTDI Chip FT32 high performance 32-bit RISC architecture */
        // #define EM_MOXIE        223     /* Moxie processor family */
        // #define EM_AMDGPU 	224 	/* AMD GPU architecture */
        // #define EM_RISCV 	243 	/* RISC-V */
        // #define EM_LANAI	244	/* Lanai 32-bit processor.  */
        // #define EM_CEVA		245	/* CEVA Processor Architecture Family */
        // #define EM_CEVA_X2	246	/* CEVA X2 Processor Family */
        // #define EM_BPF		247	/* Linux BPF – in-kernel virtual machine.  */
        // #define EM_GRAPHCORE_IPU 248	/* Graphcore Intelligent Processing Unit */
        // #define EM_IMG1		249	/* Imagination Technologies */
        // #define EM_NFP		250	/* Netronome Flow Processor.  */
        // #define EM_VE		251	/* NEC Vector Engine */
        // #define EM_CSKY		252	/* C-SKY processor family.  */
        // #define EM_ARC_COMPACT3_64 253	/* Synopsys ARCv2.3 64-bit */
        // #define EM_MCS6502	254	/* MOS Technology MCS 6502 processor */
        // #define EM_ARC_COMPACT3	255	/* Synopsys ARCv2.3 32-bit */
        // #define EM_KVX		256	/* Kalray VLIW core of the MPPA processor family */
        // #define EM_65816	257	/* WDC 65816/65C816 */
        // #define EM_LOONGARCH	258	/* LoongArch */
        // #define EM_KF32		259	/* ChipON KungFu32 */
        // #define EM_U16_U8CORE	260	/* LAPIS nX-U16/U8 */
        // #define EM_TACHYUM	261	/* Tachyum */
        // #define EM_56800EF	262	/* NXP 56800EF Digital Signal Controller (DSC) */
    }
}
