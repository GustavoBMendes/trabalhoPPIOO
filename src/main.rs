use std::io;
use std::io::Write;

fn cabecalho() {
	println!("_________________________________________________________");
	println!("|");
	println!("| PARADIGMA DE PROGRAMAÇÃO IMPERATIVA E ORIENTADA A OBJETOS");
	println!("| TRABALHO 01 - ALGORITMO SHUNTING YARD");
	println!("| PROFESSOR: MARCO AURÉLIO LOPES BARBOSA");
	println!("| ALUNO: GUSTAVO BELANÇON MENDES         RA 99037");
	println!("| ALUNO: LUIZ JOAQUIM ADERALDO AMICHI    RA 90568");
	println!("|_________________________________________________________");
	println!(" ");
}

//converte a expressão matemática, recebida como uma string, passando para uma lista de tokens (vetor de strings)
fn lexer<'a>(expr: &'a String) -> Vec<&'a str> {

	let mut tokens_list: Vec<&str> = Vec::new();
	let mut i = 0;
	let mut j = 0;									// ÍNDICES i E j PARA PEGAR DETERMINADO TOKEN
	for c in expr.chars() {
		
		if c == ' ' || c == '\n' {					// ENCONTROU ESPAÇO OU FIM DE LINHA, PEGAR TOKENS
			let token = &expr[j..i];
			if token != "" {
				tokens_list.push(token);
				j = i + 1;
			}
			else {
				j = i + 1;
			}
		}

		else if c == '('{							// ENCONTROU PARÊNTESES
			i += 1;
			let parenteses = &expr[j..i];
			tokens_list.push(parenteses);
			j = i;
			continue;
		}

		else if c == ')' {							// FECHA PARÊNTESES
			let parenteses = &expr[j..i];
			tokens_list.push(parenteses);
			j = i;
			i += 1;
			continue;
		}

		i += 1;										//INCREMENTAR ÍNDICE
	}

	tokens_list

}

//converte lista de tokens para a notação reversa
fn parser(tokens_list: Vec<&str>) -> Vec<String> {

	let mut stack: Vec<&str> = Vec::new();
	let mut fila: Vec<String> = Vec::new();
	for tokens in tokens_list.iter() {

		if tokens != &"*" && tokens != &"/" && tokens != &"+" && tokens != &"-" && tokens != &"(" && tokens != &")" {
			fila.push(tokens.to_string()); 			//é um número, adicionar na fila de saída
		}

		else if tokens == &"(" {
			stack.push(tokens);	//operador de maior precedencia, empilha
		}

		else if tokens == &"*" {
			while stack.last() == Some(&"*") || stack.last() == Some(&"/") {
				let op = stack[stack.len()-1];
				fila.push(op.to_string());
				stack.pop();
			}
			stack.push(tokens);
		}

		else if tokens == &"/" {
			while stack.last() == Some(&"*") || stack.last() == Some(&"/") {
				let op = stack[stack.len()-1];
				fila.push(op.to_string());
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
					fila.push(op.to_string());			//fila recebe operador
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
					fila.push(op.to_string());			//fila recebe operador
					stack.pop();
				}
				stack.push(tokens);
			}
		}

		else if tokens == &")" {
			while stack.last() != Some(&"(") {
				let op = stack[stack.len()-1];
				fila.push(op.to_string());			//fila recebe op
				stack.pop();
			}
			stack.pop();
		}	
		

	}
	//acabou lista de tokens, desempilha todo o resto da pilha e insere na fila

	while stack.len() >= 1 {
		let op = stack[stack.len()-1];
		fila.push(op.to_string());						//insere op na fila
		stack.pop();
	}
	fila
}

//recebe a notação reversa e calcula o resultado da expressão, usa uma pilha de execução para realizar as operações
fn eval_step(fila: Vec<String>) -> i64 {

	let mut pilha_execucao: Vec<i64> = Vec::new();
	let mut x = 0;
	let mut y = 0;
	let mut num1: i64;
	let mut num2: i64;

	//criação de uma cópia da fila de notação reversa, será usada para imprimir o passo a passo da execução da expressão
	let mut fila_aux: Vec<String> = fila.clone();

	//para cada numero encontrado na fila da notação reversa, ele sera inserido na pilha para realizar uma operação
	for token in fila.iter(){	
		
		//se for um número, adicionar na pilha de operações 
		if token != &"*" && token != &"/" && token != &"+" && token != &"-" {

			pilha_execucao.push(token.parse::<i64>().unwrap());
			x += 1;
			y += 1;
		}


		//caso for um operador, desempilha os dois primeiro numeros da pilha 
		//realiza a operação, empilha o resultado
		//mostra a equação com o resultado parcial

		else if pilha_execucao.len() >= 2 {

			num2 = pilha_execucao[x-1];
			pilha_execucao.pop(); 
			x -= 1;

			num1 = pilha_execucao[x-1];
			pilha_execucao.pop();

			if token == &"+" {

				let result_parc = num1 + num2;
				pilha_execucao.push(result_parc);
			}

			else if token == &"-" {

				let result_parc = num1 - num2;
				pilha_execucao.push(result_parc);
			}

			else if token == &"*" {

				let result_parc = num1 * num2;
				pilha_execucao.push(result_parc);
			}

			else if token == &"/" {

				let result_parc = num1 / num2;
				pilha_execucao.push(result_parc);
			}

			//atualização da fila de notação reversa, usando uma cópia da original
			//remove os três últimos da fila e adiciona o resultado da operação que está no topo da pilha de execução
			if fila_aux.len() >= 3 {

				y += 1;
				let valor = pilha_execucao[x-1].to_string();

				fila_aux.remove(y-1);
				fila_aux.remove(y-2);
				fila_aux.remove(y-3);
				fila_aux.insert(y-3, valor);

				y -= 2;

			}

			let fila_aux2: Vec<String> = fila_aux.clone(); 
			to_string(fila_aux2);	//chama a função para imprimir o resultado parcial da expressão
			
		}

	}

	if pilha_execucao.len() == 1 {
		pilha_execucao[0]
	}

	else {
		panic!("Atenção, a entrada digitada está no formato incorreto!");
	}

}


// RECEBE UM VETOR EM NOTAÇÃO PÓS-FIXA E RETORNA UM PRINT DA EXPRESSÃO MATEMÁTICA
fn to_string(posfixo: Vec<String>) {

	let mut pilha_execucao: Vec<String> = Vec::new();
	let mut op1: String;
	let mut op2: String;

	for element in posfixo {

		if element == "*" || element == "/" || element == "+" || element == "-" {

			op2 = pilha_execucao[pilha_execucao.len() - 1].clone();
			pilha_execucao.pop();
			op1 = pilha_execucao[pilha_execucao.len() - 1].clone();
			pilha_execucao.pop();

			let mut novo: String = String::from("(");
			novo.push_str(op1.as_str());

			if element == "*" {

				novo.push_str(" * ");	
			}
			
			else if element == "/" {

				novo.push_str(" / ");	
			}

			else if element == "+" {

				novo.push_str(" + ");	
			}

			else if element == "-" {

				novo.push_str(" - ");	
			}

			novo.push_str(op2.as_str());
			novo.push_str(")");
			pilha_execucao.push(novo);
		}

		else {
			let numero = element.clone();
			pilha_execucao.push(numero);
		}

	}
	//println!("Pilha do passo a passo");
	for expr in pilha_execucao {
		println!("{}", expr);	
	}
	
}


#[test]
fn lexer_test() {

	assert_eq!(vec!["1", "+", "3"], lexer(&"1 + 3 ".to_string()));
	assert_eq!(vec!["1", "+", "2", "*", "3"], lexer(&"1 + 2 * 3 ".to_string()));
	assert_eq!(vec!["4", "/", "2", "+", "7"], lexer(&"4 / 2 + 7 ".to_string()));
	assert_eq!(vec!["1", "+", "2", "+", "3", "*", "4"], lexer(&"1 + 2 + 3 * 4 ".to_string()));
	assert_eq!(vec!["(", "1", "+", "2", "+", "3", ")", "*", "4"], lexer(&"(1 + 2 + 3) * 4 ".to_string()));
	assert_eq!(vec!["(", "10", "/", "3", "+", "23", ")", "*", "(", "1", "-", "4", ")"], lexer(&"(10 / 3 + 23) * (1 - 4) ".to_string()));
	assert_eq!(vec!["(", "(", "1", "+", "3", ")", "*", "8", "+", "1", ")", "/", "3"], lexer(&"((1 + 3) * 8 + 1) / 3 ".to_string()));
	assert_eq!(vec!["58", "-", "-8", "*", "(", "58", "+", "31", ")", "-", "-14"], lexer(&"58 - -8 * (58 + 31) - -14 ".to_string()));
	assert_eq!(vec!["-71", "*", "(", "-76", "*", "91", "*", "(", "10", "-", "5", "-", "-82", ")", "-", "-79", ")"], lexer(&"-71 * (-76 * 91 * (10 - 5 - -82) - -79) ".to_string()));
	assert_eq!(vec!["10", "*", "20", "+", "3", "*", "7", "+", "2", "*", "3", "+", "10", "/", "3", "*", "4"], lexer(&"10 * 20 + 3 * 7 + 2 * 3 + 10 / 3 * 4 ".to_string()));
	assert_eq!(vec!["(", "-13", "-", "-73", ")", "*", "(", "44", "-", "-78", "-", "77", "+", "42", "-", "-32", ")"], lexer(&"(-13 - -73) * (44 - -78 - 77 + 42 - -32) ".to_string()));
	assert_eq!(vec!["-29", "*", "49", "+", "47", "-", "29", "+", "74", "-", "-85", "-", "-27", "+", "4", "-", "28"], lexer(&"-29 * 49 + 47 - 29 + 74 - -85 - -27 + 4 - 28 ".to_string()));
	assert_eq!(vec!["-74", "-", "-14", "+", "42", "-", "-4", "+", "-78", "+", "-50", "*", "-35", "*", "-81", "+", "-41"], lexer(&"-74 - -14 + 42 - -4 + -78 + -50 * -35 * -81 + -41 ".to_string()));
	assert_eq!(vec!["80", "*", "-18", "*", "(", "85", "*", "(", "-46", "+", "-71", ")", "-", "12", "+", "26", "-", "59", ")", "+", "84"], lexer(&"80 * -18 * (85 * (-46 + -71) - 12 + 26 - 59) + 84 ".to_string()));
	assert_eq!(vec!["25", "+", "38", "+", "88", "+", "(", "-6", "-", "-73", ")", "*", "(", "-83", "+", "(", "53", "+", "97", ")", "*", "14", ")"], lexer(&"25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14) ".to_string()));
	assert_eq!(vec!["(", "84", "-", "90", ")", "*", "(", "-8", "-", "75", "+", "-83", "*", "(", "56", "-", "-77", ")", "+", "4", "+", "-94", ")"], lexer(&"(84 - 90) * (-8 - 75 + -83 * (56 - -77) + 4 + -94) ".to_string()));
	assert_eq!(vec!["(", "54", "-", "-8", "-", "-35", "+", "-68", "-", "-90", ")", "*", "-39", "+", "-43", "+", "-91", "*", "-30"], lexer(&"(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30 ".to_string()));
	assert_eq!(vec!["-13", "-", "-74", "+", "(", "66", "+", "-57", ")", "*", "-93", "*", "-9", "*", "77", "+", "79", "-", "66", "+", "-53"], lexer(&"-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53 ".to_string()));
	assert_eq!(vec!["(", "-72", "-", "50", "*", "-74", "+", "-45", ")", "*", "92", "*", "21", "*", "5", "*", "(", "-13", "-", "66", "-", "18", ")"], lexer(&"(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18) ".to_string()));
	assert_eq!(vec!["-7", "-", "-37", "*", "(", "90", "+", "70", ")", "-", "30", "-", "-44", "+", "-32", "-", "56", "-", "-48", "-", "-78"], lexer(&"-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78 ".to_string()));
	assert_eq!(vec!["65", "*", "-83", "-", "-3", "+", "-20", "+", "24", "-", "85", "*", "(", "-24", "+", "-32", ")", "*", "(", "61", "-", "20", ")"], lexer(&"65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20) ".to_string()));
	assert_eq!(vec!["55", "*", "48", "*", "-44", "-", "-32", "+", "1", "*", "-80", "*", "-94", "-", "74", "*", "-53", "+", "-30", "+", "-61"], lexer(&"55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61 ".to_string()));
	assert_eq!(vec!["-82", "*", "(", "25", "+", "62", "+", "3", ")", "-", "-72", "+", "-65", "*", "-32", "*", "(", "77", "+", "12", ")", "-", "-95", "+", "51"], lexer(&"-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51 ".to_string()));
	assert_eq!(vec!["(", "2", "-", "65", "-", "(", "-24", "+", "-97", ")", "*", "-5", "*", "-61", ")", "*", "(", "-41", "+", "85", "*", "9", "*", "-92", "*", "(", "75", "-", "18", ")", ")"], lexer(&"(2 - 65 - (-24 + -97) * -5 * -61) * (-41 + 85 * 9 * -92 * (75 - 18)) ".to_string()));
	assert_eq!(vec!["-20", "+", "-51", "+", "20", "+", "-68", "*", "-11", "+", "-35", "*", "-14", "-", "95", "-", "32", "+", "-52", "*", "-23", "-", "-90", "*", "-42"], lexer(&"-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42 ".to_string()));
}

#[test]
fn parser_test() {

	assert_eq!(vec!["1", "3", "+"], parser(vec!["1", "+", "3"]));
	assert_eq!(vec!["1", "2", "3", "*", "+"], parser(vec!["1", "+", "2", "*", "3"]));
	assert_eq!(vec!["4", "2", "/", "7", "+"], parser(vec!["4", "/", "2", "+", "7"]));
	assert_eq!(vec!["1", "2", "+", "3", "4", "*", "+"], parser(vec!["1", "+", "2", "+", "3", "*", "4"]));
	assert_eq!(vec!["1", "2", "+", "3", "+", "4", "*"], parser(vec!["(", "1", "+", "2", "+", "3", ")", "*", "4"]));
	assert_eq!(vec!["10", "3", "/", "23", "+", "1", "4", "-", "*"], parser(vec!["(", "10", "/", "3", "+", "23", ")", "*", "(", "1", "-", "4", ")"]));
	assert_eq!(vec!["1", "3", "+", "8", "*", "1", "+", "3", "/"], parser(vec!["(", "(", "1", "+", "3", ")", "*", "8", "+", "1", ")", "/", "3"]));
	assert_eq!(vec!["58", "-8", "58", "31", "+", "*", "-", "-14", "-"], parser(vec!["58", "-", "-8", "*", "(", "58", "+", "31", ")", "-", "-14"]));
	assert_eq!(vec!["-71", "-76", "91", "*", "10", "5", "-", "-82", "-", "*", "-79", "-", "*"], parser(vec!["-71", "*", "(", "-76", "*", "91", "*", "(", "10", "-", "5", "-", "-82", ")", "-", "-79", ")"]));
	assert_eq!(vec!["10", "20", "*", "3", "7", "*", "+", "2", "3", "*", "+", "10", "3", "/", "4", "*", "+"], parser(vec!["10", "*", "20", "+", "3", "*", "7", "+", "2", "*", "3", "+", "10", "/", "3", "*", "4"]));
	assert_eq!(vec!["-13", "-73", "-", "44", "-78", "-", "77", "-", "42", "+", "-32", "-", "*"], parser(vec!["(", "-13", "-", "-73", ")", "*", "(", "44", "-", "-78", "-", "77", "+", "42", "-", "-32", ")"]));
	assert_eq!(vec!["-29", "49", "*", "47", "+", "29", "-", "74", "+", "-85", "-", "-27", "-", "4", "+", "28", "-"], parser(vec!["-29", "*", "49", "+", "47", "-", "29", "+", "74", "-", "-85", "-", "-27", "+", "4", "-", "28"]));
	assert_eq!(vec!["-74", "-14", "-", "42", "+", "-4", "-", "-78", "+", "-50", "-35", "*", "-81", "*", "+", "-41", "+"], parser(vec!["-74", "-", "-14", "+", "42", "-", "-4", "+", "-78", "+", "-50", "*", "-35", "*", "-81", "+", "-41"]));
	assert_eq!(vec!["80", "-18", "*", "85", "-46", "-71", "+", "*", "12", "-", "26", "+", "59", "-", "*", "84", "+"], parser(vec!["80", "*", "-18", "*", "(", "85", "*", "(", "-46", "+", "-71", ")", "-", "12", "+", "26", "-", "59", ")", "+", "84"]));
	assert_eq!(vec!["25", "38", "+", "88", "+", "-6", "-73", "-", "-83", "53", "97", "+", "14", "*", "+", "*", "+"], parser(vec!["25", "+", "38", "+", "88", "+", "(", "-6", "-", "-73", ")", "*", "(", "-83", "+", "(", "53", "+", "97", ")", "*", "14", ")"]));
	assert_eq!(vec!["84", "90", "-", "-8", "75", "-", "-83", "56", "-77", "-", "*", "+", "4", "+", "-94", "+", "*"], parser(vec!["(", "84", "-", "90", ")", "*", "(", "-8", "-", "75", "+", "-83", "*", "(", "56", "-", "-77", ")", "+", "4", "+", "-94", ")"]));
	assert_eq!(vec!["54", "-8", "-", "-35", "-", "-68", "+", "-90", "-", "-39", "*", "-43", "+", "-91", "-30", "*", "+"], parser(vec!["(", "54", "-", "-8", "-", "-35", "+", "-68", "-", "-90", ")", "*", "-39", "+", "-43", "+", "-91", "*", "-30"]));
	assert_eq!(vec!["-13", "-74", "-", "66", "-57", "+", "-93", "*", "-9", "*", "77", "*", "+", "79", "+", "66", "-", "-53", "+"], parser(vec!["-13", "-", "-74", "+", "(", "66", "+", "-57", ")", "*", "-93", "*", "-9", "*", "77", "+", "79", "-", "66", "+", "-53"]));
	assert_eq!(vec!["-72", "50", "-74", "*", "-", "-45", "+", "92", "*", "21", "*", "5", "*", "-13", "66", "-", "18", "-", "*"], parser(vec!["(", "-72", "-", "50", "*", "-74", "+", "-45", ")", "*", "92", "*", "21", "*", "5", "*", "(", "-13", "-", "66", "-", "18", ")"]));
	assert_eq!(vec!["-7", "-37", "90", "70", "+", "*", "-", "30", "-", "-44", "-", "-32", "+", "56", "-", "-48", "-", "-78", "-"], parser(vec!["-7", "-", "-37", "*", "(", "90", "+", "70", ")", "-", "30", "-", "-44", "+", "-32", "-", "56", "-", "-48", "-", "-78"]));
	assert_eq!(vec!["65", "-83", "*", "-3", "-", "-20", "+", "24", "+", "85", "-24", "-32", "+", "*", "61", "20", "-", "*", "-"], parser(vec!["65", "*", "-83", "-", "-3", "+", "-20", "+", "24", "-", "85", "*", "(", "-24", "+", "-32", ")", "*", "(", "61", "-", "20", ")"]));
	assert_eq!(vec!["55", "48", "*", "-44", "*", "-32", "-", "1", "-80", "*", "-94", "*", "+", "74", "-53", "*", "-", "-30", "+", "-61", "+"], parser(vec!["55", "*", "48", "*", "-44", "-", "-32", "+", "1", "*", "-80", "*", "-94", "-", "74", "*", "-53", "+", "-30", "+", "-61"]));
	assert_eq!(vec!["-82", "25", "62", "+", "3", "+", "*", "-72", "-", "-65", "-32", "*", "77", "12", "+", "*", "+", "-95", "-", "51", "+"], parser(vec!["-82", "*", "(", "25", "+", "62", "+", "3", ")", "-", "-72", "+", "-65", "*", "-32", "*", "(", "77", "+", "12", ")", "-", "-95", "+", "51"]));
	assert_eq!(vec!["2", "65", "-", "-24", "-97", "+", "-5", "*", "-61", "*", "-", "-41", "85", "9", "*", "-92", "*", "75", "18", "-", "*", "+", "*"], parser(vec!["(", "2", "-", "65", "-", "(", "-24", "+", "-97", ")", "*", "-5", "*", "-61", ")", "*", "(", "-41", "+", "85", "*", "9", "*", "-92", "*", "(", "75", "-", "18", ")", ")"]));
	assert_eq!(vec!["-20", "-51", "+", "20", "+", "-68", "-11", "*", "+", "-35", "-14", "*", "+", "95", "-", "32", "-", "-52", "-23", "*", "+", "-90", "-42", "*", "-"], parser(vec!["-20", "+", "-51", "+", "20", "+", "-68", "*", "-11", "+", "-35", "*", "-14", "-", "95", "-", "32", "+", "-52", "*", "-23", "-", "-90", "*", "-42"]));

}

#[test]
fn eval_step_test() {

	assert_eq!(4, eval_step(vec!["1", "3", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(7, eval_step(vec!["1", "2", "3", "*", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(9, eval_step(vec!["4", "2", "/", "7", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(15, eval_step(vec!["1", "2", "+", "3", "4", "*", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(24, eval_step(vec!["1", "2", "+", "3", "+", "4", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(-78, eval_step(vec!["10", "3", "/", "23", "+", "1", "4", "-", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(11, eval_step(vec!["1", "3", "+", "8", "*", "1", "+", "3", "/"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(784, eval_step(vec!["58", "-8", "58", "31", "+", "*", "-", "-14", "-"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(42714523, eval_step(vec!["-71", "-76", "91", "*", "10", "5", "-", "-82", "-", "*", "-79", "-", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(239, eval_step(vec!["10", "20", "*", "3", "7", "*", "+", "2", "3", "*", "+", "10", "3", "/", "4", "*", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(7140, eval_step(vec!["-13", "-73", "-", "44", "-78", "-", "77", "-", "42", "+", "-32", "-", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(-1241, eval_step(vec!["-29", "49", "*", "47", "+", "29", "-", "74", "+", "-85", "-", "-27", "-", "4", "+", "28", "-"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(-141883, eval_step(vec!["-74", "-14", "-", "42", "+", "-4", "-", "-78", "+", "-50", "-35", "*", "-81", "*", "+", "-41", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(14385684, eval_step(vec!["80", "-18", "*", "85", "-46", "-71", "+", "*", "12", "-", "26", "+", "59", "-", "*", "84", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(135290, eval_step(vec!["25", "38", "+", "88", "+", "-6", "-73", "-", "-83", "53", "97", "+", "14", "*", "+", "*", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(67272, eval_step(vec!["84", "90", "-", "-8", "75", "-", "-83", "56", "-77", "-", "*", "+", "4", "+", "-94", "+", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(-1954, eval_step(vec!["54", "-8", "-", "-35", "-", "-68", "+", "-90", "-", "-39", "*", "-43", "+", "-91", "-30", "*", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(580062, eval_step(vec!["-13", "-74", "-", "66", "-57", "+", "-93", "*", "-9", "*", "77", "*", "+", "79", "+", "66", "-", "-53", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(-3357342660, eval_step(vec!["-72", "50", "-74", "*", "-", "-45", "+", "92", "*", "21", "*", "5", "*", "-13", "66", "-", "18", "-", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(5965, eval_step(vec!["-7", "-37", "90", "70", "+", "*", "-", "30", "-", "-44", "-", "-32", "+", "56", "-", "-48", "-", "-78", "-"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(189772, eval_step(vec!["65", "-83", "*", "-3", "-", "-20", "+", "24", "+", "85", "-24", "-32", "+", "*", "61", "20", "-", "*", "-"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(-104777, eval_step(vec!["55", "48", "*", "-44", "*", "-32", "-", "1", "-80", "*", "-94", "*", "+", "74", "-53", "*", "-", "-30", "+", "-61", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(177958, eval_step(vec!["-82", "25", "62", "+", "3", "+", "*", "-72", "-", "-65", "-32", "*", "77", "12", "+", "*", "+", "-95", "-", "51", "+"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(-147799088242, eval_step(vec!["2", "65", "-", "-24", "-97", "+", "-5", "*", "-61", "*", "-", "-41", "85", "9", "*", "-92", "*", "75", "18", "-", "*", "+", "*"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
	assert_eq!(-1524, eval_step(vec!["-20", "-51", "+", "20", "+", "-68", "-11", "*", "+", "-35", "-14", "*", "+", "95", "-", "32", "-", "-52", "-23", "*", "+", "-90", "-42", "*", "-"].iter().map(|x| x.to_string()).collect::<Vec<String>>()));
}

fn main() {

	cabecalho();

	let mut expr = String::new();
	println!("******Leitura de expressões aritméticas******");
	println!("");
	println!("Entre com exepressão a ser lida: ");
	print!("> ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut expr)
		.expect("Falha na leitura da expressão!");
	
	//println!("Dividindo a expressão em tokens e inserindo em uma lista.");

	let tokens_list: Vec<&str> = lexer(&expr);	// LISTA DE TOKENS

	//println!("Imprimindo tokens da lista...");
	//println!("{:?}", tokens_list);

	//println!("Criando a pilha de operadores e a fila de saída dos tokens...");
	
	let fila: Vec<String> = parser(tokens_list);

	//println!("Imprimindo a notação reversa");
	//println!("{:?}", fila);

	//println!("Criando pilha de execução dos operandos para o cálculo do resultado final");
	
	eval_step(fila);
	//println!("Resposta final = {}", resposta);

}