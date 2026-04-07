
mod data_structures;
mod phonenumbers;
mod rat_in_a_maze;
mod n_queen;

fn main() {
    //reverse_array::run()
    //hourglass::run()
    //data_structures::arrays::sparse_arrays::run();
    //data_structures::rotten_tomatos::run();
    //let a = phonenumbers::run("20565855".to_string());
    //println!("Combinations {a:?}")

    /*rat_in_a_maze::run(
        vec![
            vec![1,0,0,0],
            vec![1,1,0,1],
            vec![1,1,0,0],
            vec![0,1,1,1]
        ]
    )*/

    let v = n_queen::run(4);
}