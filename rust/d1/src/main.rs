use std::{path::Path, fs::File, io::Read};

fn main() {
    let path = Path::new("../../input1");
    let mut file = File::open(&path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();


    let mut v:Vec<i32> = Vec::new();   
    let mut c = 0;
    v.push(0);
    for x in s.trim().lines() {
        match x {
            "" => {
                c = c + 1; 
                v.push(0);
            },
            _ => v[c] = v[c] + x.parse::<i32>().unwrap(),
        };
    }
    v.sort_by(|a,b|b.cmp(a));
    print!("{}", v[0] + v[1] + v[2]);
}
