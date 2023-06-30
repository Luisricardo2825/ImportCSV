use std::env;

pub fn get_params() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    for ele in args.chunks(2) {
        println!("{:?}", ele);
        if ele.len() >= 2 {
            let arg = ele.get(0).unwrap();
            if arg.contains("--") {
                let arg_value = ele.get(1).unwrap();
                println!("a: {} - b: {}", ele.get(0).unwrap(),);
            }
        }
        println!("{:?}", ele);
    }
}
