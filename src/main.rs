use std::io::{self, Write};

fn main() {
    input_fields();
}

//Entrada de dados da array
fn input_fields(){
    let mut array = vec![];
    let mut exit = String::from("N");
    
    while exit.trim().to_uppercase() != "S" {
        let input = inputs("Informe algo para sua String:");
        
        if input != "" {
            array.push(input.trim().to_string());
            println!("{} adicionado ao array", input.trim());

            exit.clear();
            exit = inputs("Deseja parar? (S/N):");
        }
    }
    
    array_mananger(&mut array)
}

//Gerenciamento da array do usuário
fn array_mananger(array:&mut Vec<String>){
    if array.len() == 0 {
        println!("Sua array está vazia");
        return;
    }

    println!("Sua array: {:?}", array);

    loop {
        let mut options = inputs(
            "O que deseja fazer? \n
            EX - Apagar ultimo elemento\n
            RE - para remover elemento determinado\n
            EXALL - Para apagar array inteira\n
            O - Organizar array\n
            S - Para sair"
        );
        
        match options.trim().to_uppercase().as_str(){
            "EX" => {
                array.pop();
                println!("Sua nova array: {:?}", array);
            },
            "RE" => {
                let indice = inputs("Qual iten deseja remover?");
                let mut x:usize = indice.parse().expect("Erro ao converter");
                if x > 0 {
                    x = x - 1
                } else {
                    x = 0
                }
                array.remove(x);
                println!("Sua array: {:?}", array);
            },
            "EXALL" => {
                array.clear();
                println!("Array apagada completamente, fechando aplicação");
                break;
            }
            "O" => {
                array.sort();
                println!("Sua array organziada: {:?}", array);
            },
            "S" => break,
            _ => println!("Nenhuma opção válida selecionada"),
            
        }
        
        options.clear();
    }
}

//Função auxiliar para entrada de propts e saida de dados;
fn inputs(prompt:&str) -> String{
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Falha ao ler linha");
    buffer.trim().to_string()
}
