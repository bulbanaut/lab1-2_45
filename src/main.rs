use std::{io::stdin, f64::consts::PI};

/*
TODO:добавить комментарии
a, b, c - стороны треугольника; gamma - прямой угол; alpha, beta - острые углы (в радианах), h - высота опущенная на гипотенузу c; S - площадь; P - периметр
*/

fn main() {
    println!("Выберите при помощи каких переменных будут проводится вычисления: 1)a, b; 2)a, c; 3)a, h; 4)b, Альфа 5)h, Бета; 6)c, Бета");
    let selection: f64 = read_var();
    if selection == 1.0 {
        calc1();
    } else if selection == 2.0 {
        calc2();
    } else if selection == 3.0 {
        calc3();
    } else if selection == 4.0 {
        calc4();
    } else if selection == 5.0 {
        calc5();
    } else if selection == 6.0 {
        calc6();
    } else {
        println!("Введено число не соответствующее множеству Q[1;6]");
        main();
    }
    pause();
}

fn calc1() {
    println!("Введите значение a и b");
    let a: f64 = read_var();
    let b: f64 = read_var();

    let c: f64 = calc_hypo(a, b);
    
    calc_angle(a, b);

    calc_h(a, b, c);

    calc_s(a, b);

    calc_p(a, b, c);
}

fn calc2() {
    println!("Введите значение a и c");
    let a: f64 = read_var();
    let c: f64 = read_var();

    let b: f64 = (c.powi(2) - a.powi(2)).sqrt();
    println!("b: {b}");

    calc_angle(a, b);

    calc_h(a, b, c);

    calc_s(a, b);

    calc_p(a, b, c);
}

fn calc3() {
    println!("Введите значение a и h");
    let a: f64 = read_var();
    let h: f64 = read_var();

    let cm: f64 = (a.powi(2) - h.powi(2)).sqrt(); //вычисляем отрезок гипотенузы
    let b: f64 = h * (a/cm);

    let c:f64 = calc_hypo(a, b);

    calc_angle(a, b);

    calc_s(a, b);

    calc_p(a, b, c);
}

fn calc4() {
    println!("Введите значение b и альфа");
    let b: f64 = read_var();
    let alpha: f64 = read_var();

    let beta: f64 = 90.0 - b;
    println!("Beta: {beta}");

    let a: f64 = b * alpha.tan();
    println!("A: {a}");

    let c: f64 = calc_hypo(a, b);

    calc_h(a, b, c);

    calc_s(a, b);

    calc_p(a, b, c);
}

fn calc5() {
    println!("Введите значение h и бета");
    let h: f64 = read_var();
    let beta: f64 = read_var();

    let alpha: f64 = 90.0 - beta;
    println!("Альфа: {alpha}");

    let a: f64 = h / beta.sin();
    println!("a: {a}");

    let cm: f64 = (a.powi(2) - h.powi(2)).sqrt();

    let b: f64 = h * (a/cm);
    println!("b: {b}");

    let c: f64 = cm * (a/cm);
    println!("c: {c}");

    calc_hypo(a, b);

    calc_p(a, b, c);

    calc_s(a, b);
}

fn calc6() {
    println!("Введите значение c и бета");
    let c: f64 = read_var();
    let beta: f64 = read_var();

    let alpha: f64 = 90.0 - beta;
    println!("Альфа: {alpha}");

    let a: f64 = c * beta.cos();
    println!("a: {a}");

    let b: f64 = c * alpha.cos();
    println!("b: {b}");

    calc_h(a, b, c);

    calc_s(a, b);

    calc_p(a, b, c);
}

fn calc_hypo(a:f64, b:f64) -> f64 {
    let c: f64 = (a.powi(2) + b.powi(2)).sqrt();
    println!("c: {c}");
    c
}

fn calc_angle(a:f64, b:f64) {
    let alpha: f64 = rad2d((a/b).atan());
    let beta: f64 = rad2d((b/a).atan());
    println!("Альфа: {alpha}");
    println!("Бета: {beta}");
}

fn calc_s(a:f64, b:f64) {
    let s: f64 = (a*b)/2.0;
    println!("Площадь: {s}");
}

fn calc_h(a:f64, b:f64, c:f64) {
    let h: f64 = (a*b)/c;
    println!("h: {h}");
}

fn calc_p(a:f64, b:f64, c:f64) {
    let p: f64 = a + b + c;
    println!("Периметр: {p}");
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

fn pause() { //фукция паузы
    println!("нажмите Enter чтобы выйти.");
    let mut q = String::new();
    stdin().read_line(&mut q).expect("ошибка");
}