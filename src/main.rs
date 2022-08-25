use std::io;


fn convert_to_int(data_input: & String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    //imutável
    // let name: &str = "Amanda";
    //transformar let para mutável
    // let mut age = 42;
    // println!("Hello, {}!", name);

    //tipos de dados

    // let x: i64 = 23; //padrão i32
    
    // let y: u32 = 25; //u -> só permite números positivos

    // let f: f32 = 5.6; //numeros com ponto flutuante

    // let b: bool = false;


    //loops e condicionais

    // let number1 = 24;
    // let number2 = 22;

    // if number1 > number2 {
    //     println!("{} > {}", number1, number2);
    // }else{
    //     println!("{} < {}", number1, number2);
    // }


    //input
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Erro ao ler numero 1");

    let mut num2 = String::new();
    io::stdin().read_line(&mut num1).expect("Erro ao ler numero 2");

    if convert_to_int(&num1) > convert_to_int(&num2){
        println!("O {} é maior que {}", num1, num2);
    }else{
        println!("O {} é maior que {}", num2, num1);
    }

}
