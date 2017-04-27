extern crate quickselect;
extern crate rand;

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
    let mut vec = Vec::new();
    for i in 0 .. N_ELEM {
        vec.push(i);
    }

    'outter: loop {
        let mut rng = rand::thread_rng();
        rng.shuffle(vec.as_mut_slice());


        let mut nths = Vec::new();
        for index in 0 .. N_ELEM / 10 {
            let rand_index = 10 * index + (rand::random::<usize>() % 10);
            let choosen = vec[rand_index];
            nths.push(choosen);
        }

        quicksort(nths.as_mut_slice());

        quickselect_multiple(vec.as_mut_slice(), nths.as_slice(), 0);

        for i in 0 .. nths.len() {
            let index = nths[i];
            if index != vec[index] {
                println!("{}: {}", index, vec[index]);
                break 'outter;
            }
        }
    }

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
