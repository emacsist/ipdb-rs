use ipdbv4_rust::find;

fn main() {
    let v = find("58.250.137.36", "CN");
    println!("{:?}", v);
    let mut s = 0;
    for _i in 0..1000000 {
        if let Ok(_v) = find("58.250.137.36", "CN") {
            s += 1;
        }
    }
    println!("ok {}, err {}", s, 1000000 - s);
}
