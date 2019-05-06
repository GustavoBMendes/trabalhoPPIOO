extern crate queues;
use std::io;
use std::collections::LinkedList;
use queues::*;

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
			list.push_back(tokens);
			j = i+1;
    	}

		else if c == '('{                   //encontrou parenteses
			i += 1;
			let parenteses = &expr[j..i];
    		list.push_back(parenteses);
    		j = i;
			continue;
		}

		else if c == ')' {                  //fecha parenteses
			let parenteses = &expr[j..i];
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

	println!("Criando a pilha de operadores e a fila de saída dos tokens...");
	let mut stack = LinkedList::new();
	let mut fila: Queue<&str> = queue![];
	for tokens in list.iter_mut() {
		let topo_pilha = stack.back_mut();
		if tokens != &"*" || tokens != &"/" || tokens != &"+" || tokens != &"-" || tokens != &"(" || tokens != &")" {
			fila.add(tokens); 			//é um número, adicionar na fila de saída
		}

		else if tokens == &"*" || tokens == &"/" || tokens == &"(" {
			stack.push_back(tokens);	//operadores de maior precedencia, empilha
		}

		else if tokens == &"+" || tokens == &"-" {
			if stack.is_empty() || stack.back_mut() != "*" || stack.back_mut() != "/"{
				stack.push_back(tokens);	//operadores de menor precedencia
											//empilha caso nao existe um de maior precedencia no topo
			}

			else {
				let mult: Option<&mut &str> = Some("*");
				while stack.back_mut() ==  mult || stack.back_mut() == "/" {
					let mut op = stack.pop_back();
					fila.add(op);			//fila recebe op
				}
			}
		}

		else if tokens == &")" {
			while stack.back_mut() != &"(" {
				let mut op = stack.pop_back();
				fila.add(op);				//fila recebe op
			}
			stack.pop_back();
		}	

	}
	//acabou lista de tokens, desempilha todo o resto da pilha e insere na fila
	while !(stack.is_empty()) {
		let mut op = stack.pop_back();
		fila.add(op);						//insere op na fila
	}
	
	for operators in stack.iter_mut() {
		println!("{}", operators);
	}

}
