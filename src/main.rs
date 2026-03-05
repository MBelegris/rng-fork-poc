use rand::RngCore;

fn main() {
    // Warm up the RNG before forking. 
    // ** REQUIRED FOR THIS TO WORK **
    let mut warmup = rand::thread_rng();
    let pre = (0..4).map(|_| warmup.next_u64()).collect::<Vec<_>>();
    eprintln!("pre-fork warmup (parent pid={}): {:?}", unsafe { libc::getpid() }, pre);

    // Fork the process. The child inherits a copy of the parent's memory,
    // including thread-local RNG state.
    let pid = unsafe { libc::fork() };
    if pid < 0 {
        eprintln!("fork failed");
        std::process::exit(1);
    }

    // "Re-instantiate" thread_rng() after fork in both parent and child.
    // If RNG state was copied, both will often output the same sequence here.
    let mut rng = rand::thread_rng();
    let seq = (0..8).map(|_| rng.next_u64()).collect::<Vec<_>>();

    let my_pid = unsafe { libc::getpid() };
    let role = if pid == 0 { "child " } else { "parent" };

    println!("{} pid={}: {:?}", role, my_pid, seq);

    if pid == 0 {
        // Make sure the child doesn't continue running parent code.
        std::process::exit(0);
    } else {
        // Wait for child so output is stable.
        unsafe {
            libc::waitpid(pid, std::ptr::null_mut(), 0);
        }
    }
}
