use hudhook::inject::Process;

fn main() {
    let mut cur_exe = std::env::current_exe().unwrap();
    cur_exe.push("..");
    cur_exe.push("hello_hud.dll");

    let cur_dll = cur_exe.canonicalize().unwrap();
    print!("{:?}", cur_dll);
    print!("test");
    Process::by_name("DARKSOULS.exe")
        .unwrap()
        .inject(
            "/home/eloise/Documents/Ptdetest/target/i686-pc-windows-msvc/release/hello_hud.dll"
                .into(),
        )
        .unwrap();
}
