use std::{io::stdin, f64::consts::PI, fs::read};

/*
TODO:calc с 2 по 6, оптимизация
a, b, c - стороны треугольника; gamma - прямой угол; alpha, beta - острые углы (в радианах), h - высота опущенная на гипотенузу c; S - площадь; P - периметр
*/

fn main() {
    println!("Выберите при помощи каких переменных будут проводится вычисления: 1)a, b; 2)a, c; 3)a, h; 4)b, alpha 5)h, beta; 6)c, beta");
    let selection: f64 = read_var();
    if selection == 1.0 {
        calc1();
    } else if selection == 2.0 {
        calc2();
    }
}

fn calc1 () {
    println!("Введите значение a и b");
    let a: f64 = read_var();
    let b: f64 = read_var();

    let c: f64 = (a.powi(2) + b.powi(2)).sqrt();
    println!("{c}");
    
    calc_angle(a, b);

    let h: f64 = (a*b)/c;
    println!("{h}");

    let s: f64 = (a*b)/2.0;
    println!("{s}");

    let p: f64 = a + b + c;
    println!("{p}");
}

fn calc2() {
    println!("Введите значение a и b");
    let a: f64 = read_var();
    let c: f64 = read_var();

    let b: f64 = (c.powi(2) - a.powi(2)).sqrt();
    println!("{b}");

    calc_angle(a, b);
}

fn calc_angle(a:f64, b:f64) {
    let alpha: f64 = rad2d((a/b).atan());
    let beta: f64 = rad2d((b/a).atan());
    println!("{alpha}");
    println!("{beta}");
}

fn rad2d(rad: f64) -> f64 {
    rad * (180.0/PI)
}

fn read_var() -> f64 {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки
        break x;
    }
}