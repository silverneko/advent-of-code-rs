use itertools::{chain, Itertools};
use std::io::stdin;
use std::sync::{mpsc, Condvar, Mutex};
use utils::Intcode;

type Packet = [isize; 2];

fn main() {
    let program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    let (txs, rxs): (Vec<_>, Vec<_>) =
        std::iter::repeat_with(mpsc::channel::<Packet>).take(50).unzip();
    let nat_packet = Mutex::new([-1, -1]);
    let (condvar, idle_count) = (Condvar::new(), Mutex::new(0));
    std::thread::scope(|s| {
        let txs = &txs;
        let nat_packet = &nat_packet;
        let (condvar, idle_count) = (&condvar, &idle_count);
        s.spawn(move || {
            let mut last_y = -1;
            loop {
                let idle = condvar.wait(idle_count.lock().unwrap()).unwrap();
                let [x, y] = *nat_packet.lock().unwrap();
                if *idle < 50 || (x, y) == (-1, -1) {
                    continue;
                }
                println!("NAT x={x} y={y}");
                if last_y == y {
                    std::process::exit(0);
                }
                last_y = y;
                txs[0].send([x, y]).unwrap();
                let _idle = condvar.wait_while(idle, |idle| *idle == 50).unwrap();
            }
        });
        for (i, rx) in rxs.into_iter().enumerate() {
            let mut program = program.clone();
            s.spawn(move || {
                let mut idle = 0u32;
                let output = program.run(chain![
                    std::iter::once(i as isize),
                    std::iter::repeat_with(|| {
                        // Throttle
                        std::thread::sleep(std::time::Duration::from_millis(1));
                        if let Ok([x, y]) = rx.try_recv() {
                            if idle >= 20 {
                                *idle_count.lock().unwrap() -= 1;
                                condvar.notify_one();
                            }
                            idle = 0;
                            vec![x, y].into_iter()
                        } else {
                            idle = idle.saturating_add(1);
                            if idle == 20 {
                                *idle_count.lock().unwrap() += 1;
                                condvar.notify_one();
                            }
                            vec![-1].into_iter()
                        }
                    })
                    .flatten(),
                ]);
                for (a, x, y) in output.tuple_windows() {
                    // println!("i={i} a={a}, x={x}, y={y}");
                    match a {
                        0..50 => txs[a as usize].send([x, y]).unwrap(),
                        255 => {
                            *nat_packet.lock().unwrap() = [x, y];
                            condvar.notify_one();
                        }
                        _ => { /* ignored */ }
                    }
                }
            });
        }
    });
}
