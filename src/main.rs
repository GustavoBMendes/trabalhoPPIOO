use std::io;

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

	println!("");
    println!("A expressão lida foi: {}", expr);
    
    println!("Dividindo a expressão em tokens e inserindo em uma lista.");

    let mut i = 0;
    let mut j = 0;                          		//índices i e j para pegar determinado token

	let mut tokens_list: Vec<&str> = Vec::new();		//lista de tokens
    for c in expr.chars() {
    	
    	if c == ' ' || c == '\n'{           		//encontrou espaço ou fim de linha, pegar tokens
			let token = &expr[j..i];
			tokens_list.push(token);
			j = i+1;
    	}

		else if c == '('{                   //encontrou parenteses
			i += 1;
			let parenteses = &expr[j..i];
			tokens_list.push(parenteses);
    		j = i;
			continue;
		}

		else if c == ')' {                  //fecha parenteses
			let parenteses = &expr[j..i];
			tokens_list.push(parenteses);
    		j = i;
			i += 1;
			continue;
		}

    	i += 1;                             //incrementar indice
    }

    println!("Imprimindo tokens da lista...");
	println!("{:?}", tokens_list);


// *********************** ************************************** ***********************
// *********************** PILHA DE OPERADORES E FILA DE EXECUÇÃO ***********************
// *********************** ************************************** ***********************


	println!("Criando a pilha de operadores e a fila de saída dos tokens...");
	let mut stack: Vec<&str> = Vec::new();
	let mut fila: Vec<&str> = Vec::new();
	
	//let mut k: isize = -1;

	for tokens in tokens_list.iter() {

		if tokens != &"*" && tokens != &"/" && tokens != &"+" && tokens != &"-" && tokens != &"(" && tokens != &")" {
			fila.push(tokens); 			//é um número, adicionar na fila de saída
		}

		else if tokens == &"(" {
			stack.push(tokens);	//operadores de maior precedencia, empilha
		}

		else if tokens == &"*" {
			while stack.last() == Some(&"*") || stack.last() == Some(&"/") {
				let op = stack[stack.len()-1];
				fila.push(op);
				stack.pop();
			}
			stack.push(tokens);
		}

		else if tokens == &"/" {
			while stack.last() == Some(&"*") || stack.last() == Some(&"/") {
				let op = stack[stack.len()-1];
				fila.push(op);
				stack.pop();
			}
			stack.push(tokens);
		}

		else if tokens == &"+" {
			if stack.is_empty() || (stack.last() != Some(&"*") && stack.last() != Some(&"/") && stack.last() != Some(&"-") && stack.last() != Some(&"+")) {
				stack.push(tokens);	//operadores de menor precedencia
									//empilha caso nao existe um de maior precedencia no topo
			}

			else {
				while (stack.last() ==  Some(&"*") || stack.last() == Some(&"/") || stack.last() == Some(&"-") || stack.last() == Some(&"+")) && stack.len() >= 1 {
					let op = stack[stack.len()-1];
					fila.push(op);			//fila recebe operador
					stack.pop();
				}
				stack.push(tokens);
			}
		}

		else if tokens == &"-" {
			if stack.is_empty() || (stack.last() != Some(&"*") && stack.last() != Some(&"/") && stack.last() != Some(&"+") && stack.last() != Some(&"-")) {
				stack.push(tokens);	//operadores de menor precedencia
									//empilha caso nao existe um de maior precedencia no topo
			}

			else {
				
				while stack.last() ==  Some(&"*") || stack.last() == Some(&"/") || stack.last() == Some(&"+") || stack.last() == Some(&"-") {
					let op = stack[stack.len()-1];
					fila.push(op);			//fila recebe operador
					stack.pop();
				}
				stack.push(tokens);
			}
		}

		else if tokens == &")" {
			while stack.last() != Some(&"(") {
				let op = stack[stack.len()-1];
				fila.push(op);			//fila recebe op
				stack.pop();
			}
			stack.pop();
		}	

	}
	//acabou lista de tokens, desempilha todo o resto da pilha e insere na fila
	
	println!("{}", stack.len());

	while stack.len() >= 1 {
		let op = stack[stack.len()-1];
		fila.push(op);						//insere op na fila
		stack.pop();
	}

	for f in fila.iter() {
		print!("{}, ", f);
	}
	println!("");

	println!("Criando pilha de execução dos operandos");
	let mut pilha_operacoes: Vec<i64> = Vec::new();
	let mut x = 0;
	let mut num1: i64;
	let mut num2: i64;
	//let mut result_parcSS: &str;

	//para cada numero encontrado na fila da notação reversa, ele sera inserido na pilha para realizar uma operação
	for token in fila.iter(){	
		
		//se for um número, adicionar na pilha de operações 
		if token != &"*" && token != &"/" && token != &"+" && token != &"-" {
			pilha_operacoes.push(token.parse::<i64>().unwrap());
			x += 1;
			println!("Pilha operações[{}] = {}", x, pilha_operacoes[x-1]);
		}


		//caso for um operador, desempilha os dois primeiro numeros da pilha 
		//realiza a operação, empilha o resultado
		//mostra a equação com o resultado parcial

		else if pilha_operacoes.len() >= 2 {

			num2 = pilha_operacoes[x-1];
			pilha_operacoes.pop(); 
			x -= 1;
			num1 = pilha_operacoes[x-1];
			pilha_operacoes.pop();

			if token == &"+" {
				let result_parc = num1 + num2;
				pilha_operacoes.push(result_parc);
			}

			else if token == &"-" {
				let result_parc = num1 - num2;
				pilha_operacoes.push(result_parc);
			}

			else if token == &"*" {
				let result_parc = num1 * num2;
				pilha_operacoes.push(result_parc);
			}

			else if token == &"/" {
				let result_parc = num1 / num2;
				pilha_operacoes.push(result_parc);
			}

			println!("Pilha operações[{}] = {}", x, pilha_operacoes[x-1]);
		}

	}

	for results in pilha_operacoes.iter() {
		println!("Resultado: {}", results);
	}


}
