#[macro_use] 

extern crate dudect_bencher;

extern crate sodiumoxide;
extern crate rand;

use dudect_bencher::{ctbench_main,BenchRng, Class, CtRunner};
use rand::{Rng};
use sodiumoxide::crypto::hash::sha512::hash;
use sodiumoxide::{init, randombytes};
use rand::RngCore;
fn rand_vec(rng: &mut BenchRng) -> [u8;100] {

    let mut arr = [0u8; 100];
    rng.fill_bytes(&mut arr);
    arr
}

fn seq_vec() -> [u8;100]{
    let mut arr = [0u8;100];
    for i in 0..100{
      arr[i] = i as u8;
    }
    arr
}

fn hash_test(runner: &mut CtRunner, rng: &mut BenchRng){
    init();    
    let mut inputs: Vec<[u8;100]> = Vec::new();
    let mut classes = Vec::new();

    for _ in 0..100000{
        if rng.gen::<bool>(){
            let v1 = rand_vec(rng);
            inputs.push(v1);
            classes.push(Class::Left);
        }
        else{
            let mut v1 = seq_vec();
            //let mut v2 = v1.clone();
            //v1[5] = 7;
            inputs.push(v1);
            classes.push(Class::Right);
        }

    }

    for (class, u) in classes.into_iter().zip(inputs.into_iter()) {
        // Now time how long it takes to do a vector comparison
        runner.run_one(class, || hash(&u));
    }


}


ctbench_main!(hash_test);
