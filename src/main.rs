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
    let mut j = 0;                          		//índices i e j para pegar determinado token

	let mut tokensList: Vec<&str> = Vec::new();		//lista de tokens
    for c in expr.chars() {
    	
    	if c == ' ' || c == '\n'{           		//encontrou espaço ou fim de linha, pegar tokens
			let token = &expr[j..i];
			tokensList.push(token);
			j = i+1;
    	}

		else if c == '('{                   //encontrou parenteses
			i += 1;
			let parenteses = &expr[j..i];
			tokensList.push(parenteses);
    		j = i;
			continue;
		}

		else if c == ')' {                  //fecha parenteses
			let parenteses = &expr[j..i];
			tokensList.push(parenteses);
    		j = i;
			i += 1;
			continue;
		}

    	i += 1;                             //incrementar indice
    }

    println!("Imprimindo tokens da lista...");
	println!("{:?}", tokensList);

	println!("Criando a pilha de operadores e a fila de saída dos tokens...");
	let mut stack: Vec<&str> = Vec::new();
	let mut fila: Vec<&str> = Vec::new();
	
	let mut k = 0;

	for tokens in tokensList.iter() {

		if tokens != &"*" && tokens != &"/" && tokens != &"+" && tokens != &"-" && tokens != &"(" && tokens != &")" {
			fila.push(tokens); 			//é um número, adicionar na fila de saída
		}

		else if tokens == &"*" || tokens == &"(" {
			stack.push(tokens);	//operadores de maior precedencia, empilha
			k += 1;
		}

		else if tokens == &"/" {
			while stack[k-1] == "*" {
				let mut op = stack[k-1];
				fila.push(op);
				k -= 1;
				stack.pop();
			}
			stack.push(tokens);
			k += 1;
		}

		else if tokens == &"+" || tokens == &"-" {
			if stack.is_empty() || (stack[k-1] != "*" && stack[k-1] != "/") {
				stack.push(tokens);	//operadores de menor precedencia
				k += 1;				//empilha caso nao existe um de maior precedencia no topo
			}

			else {
				
				while stack[k-1] ==  "*" || stack[k-1] == "/" {
					let mut op = stack[k-1];
					fila.push(op);			//fila recebe operador
					k -= 1;
					stack.pop();
				}
				stack.push(tokens);
				k += 1;
			}
		}

		else if tokens == &")" {
			while stack[k-1] != "(" {
				let mut op = stack[k-1];
				fila.push(op);			//fila recebe op
				k -= 1;
				stack.pop();
			}
			stack.pop();
			k -= 1;
		}	

	}
	//acabou lista de tokens, desempilha todo o resto da pilha e insere na fila
	
	println!("{}", k);

	while k >= 1 {
		let mut op = stack[k-1];
		fila.push(op);						//insere op na fila
		k -= 1;
		stack.pop();
	}

	for f in fila.iter() {
		print!("{}, ", f);
	}
	println!("");

	println!("Criando pilha de execução dos operadores");
	let mut pilhaOperacoes: Vec<i32> = Vec::new();
	let mut x = 0;
	let mut num1 = 0;
	let mut num2 = 0;
	//let mut resultParcSS: &str;

	//para cada numero encontrado na fila da notação reversa, ele sera inserido na pilha para realizar uma operação
	for token in fila.iter(){	
		
		//se for um número, adicionar na pilha de operações 
		if token != &"*" && token != &"/" && token != &"+" && token != &"-" {
			pilhaOperacoes.push(token.parse::<i32>().unwrap());
			x += 1;
			println!("Pilha operações[{}] = {}", x, pilhaOperacoes[x-1]);
		}


		//caso for um operador, desempilha os dois primeiro numeros da pilha 
		//realiza a operação, empilha o resultado
		//mostra a equação com o resultado parcial

		else if pilhaOperacoes.len() >= 2 {

			num2 = pilhaOperacoes[x-1];
			pilhaOperacoes.pop(); 
			x -= 1;
			num1 = pilhaOperacoes[x-1];
			pilhaOperacoes.pop();

			if token == &"+" {
				let resultParc = num1 + num2;
				pilhaOperacoes.push(resultParc);
			}

			else if token == &"-" {
				let resultParc = num1 - num2;
				pilhaOperacoes.push(resultParc);
			}

			else if token == &"*" {
				let resultParc = num1 * num2;
				pilhaOperacoes.push(resultParc);
			}

			else if token == &"/" {
				let resultParc = num1 / num2;
				pilhaOperacoes.push(resultParc);
			}

			//let resultParc = resultParc.to_string();
			//resultParcSS = &resultParc.to_string();
			println!("Pilha operações[{}] = {}", x, pilhaOperacoes[x-1]);
		}

	}

	for results in pilhaOperacoes.iter() {
		println!("Resultado: {}", results);
	}


}
