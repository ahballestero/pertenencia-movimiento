fn main() {
    let x: u8 = 13;
    let y: u8 = 42;
    let z: u8 = x;
    println!("{x} {y} {z}");

    let a: [i32; 3] = [1, 2, 3];
    let b: [i32; 3] = a;
    let c: (i32, char, bool) = (1, 'a', true);
    let d: (i32, char, bool) = c;
    println!("{a:?} {b:?} {c:?} {d:?}");

    let str1: &str = "Hola";
    let str2: &str = str1;
    println!("{str1} {str2}");

    let s: String = "Hola".to_string();
    let t: String = s.clone();
    println!("{s} {t}");

    let v: Vec<i32> = vec![1, 2, 3];
    let w: Vec<i32> = v.clone();
    println!("{w:?} {v:?}");
}
