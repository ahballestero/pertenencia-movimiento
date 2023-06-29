fn main() {
    let x: u8 = 13;
    let y: u8 = 42;
    let z: u8 = x;
    println!("{x} {y} {z}");

    let a = [1, 2, 3];
    let b = a;
    let c = (1, 'a', true);
    let d = c;
    println!("{a:?} {b:?} {c:?} {d:?}")
}
