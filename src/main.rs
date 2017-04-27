extern crate quickselect;
extern crate rand;
extern crate time;

use quickselect::*;
use rand::Rng;

fn main() {
    // let mut vec = Vec::new();
    // for _ in 0 .. 100 {
    //     vec.push(rand::random::<u32>());
    // }

    // {
    //     let slice = vec.as_mut_slice();

    //     quicksort(slice);

    //     if is_sorted(slice) {
    //         println!(":)");
    //     } else {
    //         println!(":(");
    //     }
    // }
    // println!("{:?}", vec);

    const N_ELEM : usize = 1000;
    const N_SEARCHES : usize = 100;
    const WINDOW_SIZE : usize = N_ELEM / N_SEARCHES;
    const N_ITER : usize = 1000;

    let mut vec = Vec::new();
    for i in 0 .. N_ELEM {
        vec.push(i);
    }

    let mut time_sum = 0.0;
    'outter: for _ in 0 .. N_ITER {
        let mut rng = rand::thread_rng();
        rng.shuffle(vec.as_mut_slice());


        let mut nths = Vec::new();
        for index in 0 .. N_SEARCHES {
            let rand_index = WINDOW_SIZE * index +
                (rand::random::<usize>() % WINDOW_SIZE);

            let choosen = vec[rand_index];
            nths.push(choosen);
        }

        quicksort(nths.as_mut_slice());

        let t0 = time::PreciseTime::now();
        // quicksort(vec.as_mut_slice());
        quickselect_multiple(vec.as_mut_slice(), nths.as_slice());
        let t1 = time::PreciseTime::now();

        let duration = t0.to(t1);
        let micros = duration.num_microseconds().unwrap();
        time_sum += micros as f64;

        for i in 0 .. nths.len() {
            let index = nths[i];
            if index != vec[index] {
                println!("{}: {}", index, vec[index]);
                break 'outter;
            }
        }
    }

    let mean = time_sum / N_ITER as f64;
    println!("Took: {}us", mean);
}


#[allow(dead_code)]
fn is_sorted<T>(slice: &[T]) -> bool where T: Ord {
    for index in 0 .. slice.len() - 1 {
        if slice[index] > slice[index + 1] {
            return false;
        }
    }

    true
}
