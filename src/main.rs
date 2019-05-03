use std::io;
use std::collections::LinkedList;


fn cabecalho() {
    println!("");
	println!("*** TRABALHO PPIOO - PROFESSOR MARCO AURÉLIO ***");
    println!("*** ALUNOS:");
    println!("--> GUSTAVO BELANÇON MENDES, RA 99037");
    println!("--> LUIZ JOAQUIM ADERALDO AMICHI, RA 90568");
    println!("");
    println!("______________________________________________");
    println!("");
}

fn main() {

	cabecalho();

    let mut expr = String::new();
    println!("******Leitura de expressões aritméticas******");
    println!("");
    println!("Entre com exepressão a ser lida: ");
    io::stdin().read_line(&mut expr)
    	.expect("Falha na leitura da expressão!");

    println!("A expressão lida foi: {}", expr);
    
    println!("Dividindo a expressão em tokens e inserindo em uma lista.");

    let mut i = 0;
    let mut j = 0;                          //índices i e j para pegar determinado token

    let mut list = LinkedList::new();       //lista de tokens

    for c in expr.chars() {
    	
    	if c == ' ' || c == '\n'{           //encontrou espaço ou fim de linha, pegar tokens
			let tokens = &expr[j..i];
			println!("{}", tokens);
			list.push_back(tokens);
			j = i+1;
    	}

		else if c == '('{                   //encontrou parenteses
			i += 1;
			let parenteses = &expr[j..i];
    		println!("{}", parenteses);
    		list.push_back(parenteses);
    		j = i;
			continue;
		}

		else if c == ')' {                  //fecha parenteses
			let parenteses = &expr[j..i];
    		println!("{}", parenteses);
    		list.push_back(parenteses);
    		j = i;
			i += 1;
			continue;
		}

    	i += 1;                             //incrementar indice
    }

    println!("Imprimindo tokens da lista...");

    for tokens in list.iter_mut() {
        println!("{}", tokens);
    }

	println!("Criando a pilha de operadores...");
	let mut stack = LinkedList::new();
	for tokens in list.iter_mut() {
		stack.push_back(tokens);
	}

	for operators in stack.iter_mut() {
		println!("{}", operators);
	}

}
