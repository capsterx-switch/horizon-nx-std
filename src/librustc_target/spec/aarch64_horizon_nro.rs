// Switch
// TODO: docs

use super::{LinkerFlavor, LinkArgs, Target, TargetOptions, PanicStrategy};

pub fn target() -> Result<Target, String> {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(LinkerFlavor::Gcc, vec![
        "-specs=/opt/devkitpro/libnx/switch.specs".to_string(),
        "-march=armv8-a".to_string(),
        "-mtune=cortex-a57".to_string(),
        "-mtp=soft".to_string(),
        "-L/opt/devkitpro/portlibs/switch/lib".to_string(),
        "-L/opt/devkitpro/libnx/lib".to_string(),
        "-L/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf/8.2.0/pic".to_string(),
        "-L/opt/devkitpro/devkitA64/aarch64-none-elf/lib/pic".to_string(),
    ]);
    let mut post_link_args = LinkArgs::new();
    post_link_args.insert(LinkerFlavor::Gcc, vec![
        "-Wl,-z,text".to_string(),
        "-Wl,-z,muldefs".to_string(),
    ]);
    let opts = TargetOptions {
        target_family: Some("unix".to_string()),
        linker: Some("aarch64-horizon-nro-ld".to_owned()),
        executables: true,
        relocation_model: "pic".to_string(),
        disable_redzone: true,
        linker_is_gnu: true,
        max_atomic_width: Some(128),
        panic_strategy: PanicStrategy::Abort,
        abi_blacklist: super::arm_base::abi_blacklist(),
        position_independent_executables: true,
        exe_suffix: ".nro".to_string(),
        requires_uwtable: true,
        no_default_libraries: false,
        pre_link_args,
        post_link_args,
        cpu: "cortex-a57".to_string(),
        .. Default::default()
    };
    Ok(Target {
        llvm_target: "aarch64-unknown-none".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        target_os: "horizon".to_string(),
        target_env: "newlib".to_string(),
        target_vendor: String::new(),
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        linker_flavor: LinkerFlavor::Gcc,
        options: opts,
    })
}
