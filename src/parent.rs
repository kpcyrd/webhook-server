use std::env;
use std::thread;
use std::sync::mpsc;
use std::process::{Command, Child, Stdio};

use sandbox;
use config::Config;

use ::Result;

pub fn run(config: Config) -> Result<()> {
    let me = env::current_exe()?;
    info!("spawning child {:?}", me);
    let (mut child, rx) = sandbox::ipc::spawn_child(&me, &config)?;

    let _first_signal = rx.recv().unwrap();
    info!("child has been setup");

    let (tx2, rx2) = mpsc::channel();
    let reaper = thread::spawn(move || {
        for msg in rx2 {
            let mut msg: Child = msg;
            info!("child reaped: {:?}", msg.wait());
        }
    });

    loop {
        let msg = match rx.recv() {
            Ok(msg) => msg,
            Err(_) => break,
        };

        info!("ipc socket -> {:?}", msg);

        if let Some(hook) = config.hooks.get(&msg.hook) {
            info!("spawning {:?} {:?}", hook.prog, hook.args);
            let mut child = Command::new(&hook.prog);
            child.stdin(Stdio::null());
            child.args(&hook.args);

            // TODO:
            // if user is configured:
            //   - call .uid
            //   - call .gid
            //   - use home as cwd if not set

            if let Some(ref cwd) = hook.cwd {
                child.current_dir(cwd);
            }

            let child = child.spawn()?;
            tx2.send(child).unwrap();
        }
    }

    let status = child.wait()?;
    println!("child terminated: {:?}", status);
    { tx2 };
    reaper.join().unwrap();

    Ok(())
}
