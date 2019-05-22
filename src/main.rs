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
fn parser(tokens_list: Vec<&str>) -> Vec<&str> {
	let mut stack: Vec<&str> = Vec::new();
	let mut fila: Vec<&str> = Vec::new();
	for tokens in tokens_list.iter() {

		if tokens != &"*" && tokens != &"/" && tokens != &"+" && tokens != &"-" && tokens != &"(" && tokens != &")" {
			fila.push(tokens); 			//é um número, adicionar na fila de saída
		}

		else if tokens == &"(" {
			stack.push(tokens);	//operador de maior precedencia, empilha
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

	while stack.len() >= 1 {
		let op = stack[stack.len()-1];
		fila.push(op);						//insere op na fila
		stack.pop();
	}
	fila
}

//recebe a notação reversa e calcula o resultado da expressão, usa uma pilha de execução para realizar as operações
fn calcular(fila: Vec<&str>) -> i64 {

	let mut pilha_execucao: Vec<i64> = Vec::new();
	let mut x = 0;
	let mut num1: i64;
	let mut num2: i64;

	//para cada numero encontrado na fila da notação reversa, ele sera inserido na pilha para realizar uma operação
	for token in fila.iter(){	
		
		//se for um número, adicionar na pilha de operações 
		if token != &"*" && token != &"/" && token != &"+" && token != &"-" {
			pilha_execucao.push(token.parse::<i64>().unwrap());
			x += 1;
			println!("Pilha operações[{}] = {}", x-1, pilha_execucao[x-1]);
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

			println!("Pilha operações[{}] = {}", x-1, pilha_execucao[x-1]);
		}

	}

	if pilha_execucao.len() == 1 {
		pilha_execucao[0]
	}

	else {
		panic!("Atenção, a entrada digitada está no formato incorreto!");
	}

}

#[test]
fn lexer_test() {
	assert_eq!(["1", "+", "3"].to_vec(), lexer(&"1 + 3 ".to_string()));
	assert_eq!(["1", "+", "2", "*", "3"].to_vec(), lexer(&"1 + 2 * 3 ".to_string()));
	assert_eq!(["4", "/", "2", "+", "7"].to_vec(), lexer(&"4 / 2 + 7 ".to_string()));
	assert_eq!(["1", "+", "2", "+", "3", "*", "4"].to_vec(), lexer(&"1 + 2 + 3 * 4 ".to_string()));
	assert_eq!(["(", "1", "+", "2", "+", "3", ")", "*", "4"].to_vec(), lexer(&"(1 + 2 + 3) * 4 ".to_string()));
	assert_eq!(["(", "10", "/", "3", "+", "23", ")", "*", "(", "1", "-", "4", ")"].to_vec(), lexer(&"(10 / 3 + 23) * (1 - 4) ".to_string()));
	assert_eq!(["(", "(", "1", "+", "3", ")", "*", "8", "+", "1", ")", "/", "3"].to_vec(), lexer(&"((1 + 3) * 8 + 1) / 3 ".to_string()));
	assert_eq!(["58", "-", "-8", "*", "(", "58", "+", "31", ")", "-", "-14"].to_vec(), lexer(&"58 - -8 * (58 + 31) - -14 ".to_string()));
	assert_eq!(["-71", "*", "(", "-76", "*", "91", "*", "(", "10", "-", "5", "-", "-82", ")", "-", "-79", ")"].to_vec(), lexer(&"-71 * (-76 * 91 * (10 - 5 - -82) - -79) ".to_string()));
	assert_eq!(["10", "*", "20", "+", "3", "*", "7", "+", "2", "*", "3", "+", "10", "/", "3", "*", "4"].to_vec(), lexer(&"10 * 20 + 3 * 7 + 2 * 3 + 10 / 3 * 4 ".to_string()));
	assert_eq!(["(", "-13", "-", "-73", ")", "*", "(", "44", "-", "-78", "-", "77", "+", "42", "-", "-32", ")"].to_vec(), lexer(&"(-13 - -73) * (44 - -78 - 77 + 42 - -32) ".to_string()));
	assert_eq!(["-29", "*", "49", "+", "47", "-", "29", "+", "74", "-", "-85", "-", "-27", "+", "4", "-", "28"].to_vec(), lexer(&"-29 * 49 + 47 - 29 + 74 - -85 - -27 + 4 - 28 ".to_string()));
	assert_eq!(["-74", "-", "-14", "+", "42", "-", "-4", "+", "-78", "+", "-50", "*", "-35", "*", "-81", "+", "-41"].to_vec(), lexer(&"-74 - -14 + 42 - -4 + -78 + -50 * -35 * -81 + -41 ".to_string()));
	assert_eq!(["80", "*", "-18", "*", "(", "85", "*", "(", "-46", "+", "-71", ")", "-", "12", "+", "26", "-", "59", ")", "+", "84"].to_vec(), lexer(&"80 * -18 * (85 * (-46 + -71) - 12 + 26 - 59) + 84 ".to_string()));
	assert_eq!(["25", "+", "38", "+", "88", "+", "(", "-6", "-", "-73", ")", "*", "(", "-83", "+", "(", "53", "+", "97", ")", "*", "14", ")"].to_vec(), lexer(&"25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14) ".to_string()));
	assert_eq!(["(", "84", "-", "90", ")", "*", "(", "-8", "-", "75", "+", "-83", "*", "(", "56", "-", "-77", ")", "+", "4", "+", "-94", ")"].to_vec(), lexer(&"(84 - 90) * (-8 - 75 + -83 * (56 - -77) + 4 + -94) ".to_string()));
	assert_eq!(["(", "54", "-", "-8", "-", "-35", "+", "-68", "-", "-90", ")", "*", "-39", "+", "-43", "+", "-91", "*", "-30"].to_vec(), lexer(&"(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30 ".to_string()));
	assert_eq!(["-13", "-", "-74", "+", "(", "66", "+", "-57", ")", "*", "-93", "*", "-9", "*", "77", "+", "79", "-", "66", "+", "-53"].to_vec(), lexer(&"-13 - -74 + (66 + -57) * -93 * -9 * 77 + 79 - 66 + -53 ".to_string()));
	assert_eq!(["(", "-72", "-", "50", "*", "-74", "+", "-45", ")", "*", "92", "*", "21", "*", "5", "*", "(", "-13", "-", "66", "-", "18", ")"].to_vec(), lexer(&"(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18) ".to_string()));
	assert_eq!(["-7", "-", "-37", "*", "(", "90", "+", "70", ")", "-", "30", "-", "-44", "+", "-32", "-", "56", "-", "-48", "-", "-78"].to_vec(), lexer(&"-7 - -37 * (90 + 70) - 30 - -44 + -32 - 56 - -48 - -78 ".to_string()));
	assert_eq!(["65", "*", "-83", "-", "-3", "+", "-20", "+", "24", "-", "85", "*", "(", "-24", "+", "-32", ")", "*", "(", "61", "-", "20", ")"].to_vec(), lexer(&"65 * -83 - -3 + -20 + 24 - 85 * (-24 + -32) * (61 - 20) ".to_string()));
	assert_eq!(["55", "*", "48", "*", "-44", "-", "-32", "+", "1", "*", "-80", "*", "-94", "-", "74", "*", "-53", "+", "-30", "+", "-61"].to_vec(), lexer(&"55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61 ".to_string()));
	assert_eq!(["-82", "*", "(", "25", "+", "62", "+", "3", ")", "-", "-72", "+", "-65", "*", "-32", "*", "(", "77", "+", "12", ")", "-", "-95", "+", "51"].to_vec(), lexer(&"-82 * (25 + 62 + 3) - -72 + -65 * -32 * (77 + 12) - -95 + 51 ".to_string()));
	assert_eq!(["(", "2", "-", "65", "-", "(", "-24", "+", "-97", ")", "*", "-5", "*", "-61", ")", "*", "(", "-41", "+", "85", "*", "9", "*", "-92", "*", "(", "75", "-", "18", ")", ")"].to_vec(), lexer(&"(2 - 65 - (-24 + -97) * -5 * -61) * (-41 + 85 * 9 * -92 * (75 - 18)) ".to_string()));
	assert_eq!(["-20", "+", "-51", "+", "20", "+", "-68", "*", "-11", "+", "-35", "*", "-14", "-", "95", "-", "32", "+", "-52", "*", "-23", "-", "-90", "*", "-42"].to_vec(), lexer(&"-20 + -51 + 20 + -68 * -11 + -35 * -14 - 95 - 32 + -52 * -23 - -90 * -42 ".to_string()));
}

#[test]
fn parser_test() {
	assert_eq!(["1", "3", "+"].to_vec(), parser(["1", "+", "3"].to_vec()));
	assert_eq!(["1", "2", "3", "*", "+"].to_vec(), parser(["1", "+", "2", "*", "3"].to_vec()));
	assert_eq!(["4", "2", "/", "7", "+"].to_vec(), parser(["4", "/", "2", "+", "7"].to_vec()));
	assert_eq!(["1", "2", "+", "3", "4", "*", "+"].to_vec(), parser(["1", "+", "2", "+", "3", "*", "4"].to_vec()));
	assert_eq!(["1", "2", "+", "3", "+", "4", "*"].to_vec(), parser(["(", "1", "+", "2", "+", "3", ")", "*", "4"].to_vec()));
	assert_eq!(["10", "3", "/", "23", "+", "1", "4", "-", "*"].to_vec(), parser(["(", "10", "/", "3", "+", "23", ")", "*", "(", "1", "-", "4", ")"].to_vec()));
	assert_eq!(["1", "3", "+", "8", "*", "1", "+", "3", "/"].to_vec(), parser(["(", "(", "1", "+", "3", ")", "*", "8", "+", "1", ")", "/", "3"].to_vec()));
	assert_eq!(["58", "-8", "58", "31", "+", "*", "-", "-14", "-"].to_vec(), parser(["58", "-", "-8", "*", "(", "58", "+", "31", ")", "-", "-14"].to_vec()));
	assert_eq!(["-71", "-76", "91", "*", "10", "5", "-", "-82", "-", "*", "-79", "-", "*"].to_vec(), parser(["-71", "*", "(", "-76", "*", "91", "*", "(", "10", "-", "5", "-", "-82", ")", "-", "-79", ")"].to_vec()));
	assert_eq!(["10", "20", "*", "3", "7", "*", "+", "2", "3", "*", "+", "10", "3", "/", "4", "*", "+"].to_vec(), parser(["10", "*", "20", "+", "3", "*", "7", "+", "2", "*", "3", "+", "10", "/", "3", "*", "4"].to_vec()));
	assert_eq!(["-13", "-73", "-", "44", "-78", "-", "77", "-", "42", "+", "-32", "-", "*"].to_vec(), parser(["(", "-13", "-", "-73", ")", "*", "(", "44", "-", "-78", "-", "77", "+", "42", "-", "-32", ")"].to_vec()));
	assert_eq!(["-29", "49", "*", "47", "+", "29", "-", "74", "+", "-85", "-", "-27", "-", "4", "+", "28", "-"].to_vec(), parser(["-29", "*", "49", "+", "47", "-", "29", "+", "74", "-", "-85", "-", "-27", "+", "4", "-", "28"].to_vec()));
	assert_eq!(["-74", "-14", "-", "42", "+", "-4", "-", "-78", "+", "-50", "-35", "*", "-81", "*", "+", "-41", "+"].to_vec(), parser(["-74", "-", "-14", "+", "42", "-", "-4", "+", "-78", "+", "-50", "*", "-35", "*", "-81", "+", "-41"].to_vec()));
	assert_eq!(["80", "-18", "*", "85", "-46", "-71", "+", "*", "12", "-", "26", "+", "59", "-", "*", "84", "+"].to_vec(), parser(["80", "*", "-18", "*", "(", "85", "*", "(", "-46", "+", "-71", ")", "-", "12", "+", "26", "-", "59", ")", "+", "84"].to_vec()));
	assert_eq!(["25", "38", "+", "88", "+", "-6", "-73", "-", "-83", "53", "97", "+", "14", "*", "+", "*", "+"].to_vec(), parser(["25", "+", "38", "+", "88", "+", "(", "-6", "-", "-73", ")", "*", "(", "-83", "+", "(", "53", "+", "97", ")", "*", "14", ")"].to_vec()));
	assert_eq!(["84", "90", "-", "-8", "75", "-", "-83", "56", "-77", "-", "*", "+", "4", "+", "-94", "+", "*"].to_vec(), parser(["(", "84", "-", "90", ")", "*", "(", "-8", "-", "75", "+", "-83", "*", "(", "56", "-", "-77", ")", "+", "4", "+", "-94", ")"].to_vec()));
	assert_eq!(["54", "-8", "-", "-35", "-", "-68", "+", "-90", "-", "-39", "*", "-43", "+", "-91", "-30", "*", "+"].to_vec(), parser(["(", "54", "-", "-8", "-", "-35", "+", "-68", "-", "-90", ")", "*", "-39", "+", "-43", "+", "-91", "*", "-30"].to_vec()));
	assert_eq!(["-13", "-74", "-", "66", "-57", "+", "-93", "*", "-9", "*", "77", "*", "+", "79", "+", "66", "-", "-53", "+"].to_vec(), parser(["-13", "-", "-74", "+", "(", "66", "+", "-57", ")", "*", "-93", "*", "-9", "*", "77", "+", "79", "-", "66", "+", "-53"].to_vec()));
	assert_eq!(["-72", "50", "-74", "*", "-", "-45", "+", "92", "*", "21", "*", "5", "*", "-13", "66", "-", "18", "-", "*"].to_vec(), parser(["(", "-72", "-", "50", "*", "-74", "+", "-45", ")", "*", "92", "*", "21", "*", "5", "*", "(", "-13", "-", "66", "-", "18", ")"].to_vec()));
	assert_eq!(["-7", "-37", "90", "70", "+", "*", "-", "30", "-", "-44", "-", "-32", "+", "56", "-", "-48", "-", "-78", "-"].to_vec(), parser(["-7", "-", "-37", "*", "(", "90", "+", "70", ")", "-", "30", "-", "-44", "+", "-32", "-", "56", "-", "-48", "-", "-78"].to_vec()));
	assert_eq!(["65", "-83", "*", "-3", "-", "-20", "+", "24", "+", "85", "-24", "-32", "+", "*", "61", "20", "-", "*", "-"].to_vec(), parser(["65", "*", "-83", "-", "-3", "+", "-20", "+", "24", "-", "85", "*", "(", "-24", "+", "-32", ")", "*", "(", "61", "-", "20", ")"].to_vec()));
	assert_eq!(["55", "48", "*", "-44", "*", "-32", "-", "1", "-80", "*", "-94", "*", "+", "74", "-53", "*", "-", "-30", "+", "-61", "+"].to_vec(), parser(["55", "*", "48", "*", "-44", "-", "-32", "+", "1", "*", "-80", "*", "-94", "-", "74", "*", "-53", "+", "-30", "+", "-61"].to_vec()));
	assert_eq!(["-82", "25", "62", "+", "3", "+", "*", "-72", "-", "-65", "-32", "*", "77", "12", "+", "*", "+", "-95", "-", "51", "+"].to_vec(), parser(["-82", "*", "(", "25", "+", "62", "+", "3", ")", "-", "-72", "+", "-65", "*", "-32", "*", "(", "77", "+", "12", ")", "-", "-95", "+", "51"].to_vec()));
	assert_eq!(["2", "65", "-", "-24", "-97", "+", "-5", "*", "-61", "*", "-", "-41", "85", "9", "*", "-92", "*", "75", "18", "-", "*", "+", "*"].to_vec(), parser(["(", "2", "-", "65", "-", "(", "-24", "+", "-97", ")", "*", "-5", "*", "-61", ")", "*", "(", "-41", "+", "85", "*", "9", "*", "-92", "*", "(", "75", "-", "18", ")", ")"].to_vec()));
	assert_eq!(["-20", "-51", "+", "20", "+", "-68", "-11", "*", "+", "-35", "-14", "*", "+", "95", "-", "32", "-", "-52", "-23", "*", "+", "-90", "-42", "*", "-"].to_vec(), parser(["-20", "+", "-51", "+", "20", "+", "-68", "*", "-11", "+", "-35", "*", "-14", "-", "95", "-", "32", "+", "-52", "*", "-23", "-", "-90", "*", "-42"].to_vec()));

}

#[test]
fn calcular_test() {
	assert_eq!(4, calcular(["1", "3", "+"].to_vec()));
	assert_eq!(7, calcular(["1", "2", "3", "*", "+"].to_vec()));
	assert_eq!(9, calcular(["4", "2", "/", "7", "+"].to_vec()));
	assert_eq!(15, calcular(["1", "2", "+", "3", "4", "*", "+"].to_vec()));
	assert_eq!(24, calcular(["1", "2", "+", "3", "+", "4", "*"].to_vec()));
	assert_eq!(-78, calcular(["10", "3", "/", "23", "+", "1", "4", "-", "*"].to_vec()));
	assert_eq!(11, calcular(["1", "3", "+", "8", "*", "1", "+", "3", "/"].to_vec()));
	assert_eq!(784, calcular(["58", "-8", "58", "31", "+", "*", "-", "-14", "-"].to_vec()));
	assert_eq!(42714523, calcular(["-71", "-76", "91", "*", "10", "5", "-", "-82", "-", "*", "-79", "-", "*"].to_vec()));
	assert_eq!(239, calcular(["10", "20", "*", "3", "7", "*", "+", "2", "3", "*", "+", "10", "3", "/", "4", "*", "+"].to_vec()));
	assert_eq!(7140, calcular(["-13", "-73", "-", "44", "-78", "-", "77", "-", "42", "+", "-32", "-", "*"].to_vec()));
	assert_eq!(-1241, calcular(["-29", "49", "*", "47", "+", "29", "-", "74", "+", "-85", "-", "-27", "-", "4", "+", "28", "-"].to_vec()));
	assert_eq!(-141883, calcular(["-74", "-14", "-", "42", "+", "-4", "-", "-78", "+", "-50", "-35", "*", "-81", "*", "+", "-41", "+"].to_vec()));
	assert_eq!(14385684, calcular(["80", "-18", "*", "85", "-46", "-71", "+", "*", "12", "-", "26", "+", "59", "-", "*", "84", "+"].to_vec()));
	assert_eq!(135290, calcular(["25", "38", "+", "88", "+", "-6", "-73", "-", "-83", "53", "97", "+", "14", "*", "+", "*", "+"].to_vec()));
	assert_eq!(67272, calcular(["84", "90", "-", "-8", "75", "-", "-83", "56", "-77", "-", "*", "+", "4", "+", "-94", "+", "*"].to_vec()));
	assert_eq!(-1954, calcular(["54", "-8", "-", "-35", "-", "-68", "+", "-90", "-", "-39", "*", "-43", "+", "-91", "-30", "*", "+"].to_vec()));
	assert_eq!(580062, calcular(["-13", "-74", "-", "66", "-57", "+", "-93", "*", "-9", "*", "77", "*", "+", "79", "+", "66", "-", "-53", "+"].to_vec()));
	assert_eq!(-3357342660, calcular(["-72", "50", "-74", "*", "-", "-45", "+", "92", "*", "21", "*", "5", "*", "-13", "66", "-", "18", "-", "*"].to_vec()));
	assert_eq!(5965, calcular(["-7", "-37", "90", "70", "+", "*", "-", "30", "-", "-44", "-", "-32", "+", "56", "-", "-48", "-", "-78", "-"].to_vec()));
	assert_eq!(189772, calcular(["65", "-83", "*", "-3", "-", "-20", "+", "24", "+", "85", "-24", "-32", "+", "*", "61", "20", "-", "*", "-"].to_vec()));
	assert_eq!(-104777, calcular(["55", "48", "*", "-44", "*", "-32", "-", "1", "-80", "*", "-94", "*", "+", "74", "-53", "*", "-", "-30", "+", "-61", "+"].to_vec()));
	assert_eq!(177958, calcular(["-82", "25", "62", "+", "3", "+", "*", "-72", "-", "-65", "-32", "*", "77", "12", "+", "*", "+", "-95", "-", "51", "+"].to_vec()));
	assert_eq!(-147799088242, calcular(["2", "65", "-", "-24", "-97", "+", "-5", "*", "-61", "*", "-", "-41", "85", "9", "*", "-92", "*", "75", "18", "-", "*", "+", "*"].to_vec()));
	assert_eq!(-1524, calcular(["-20", "-51", "+", "20", "+", "-68", "-11", "*", "+", "-35", "-14", "*", "+", "95", "-", "32", "-", "-52", "-23", "*", "+", "-90", "-42", "*", "-"].to_vec()));
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

	println!(" ");
	
	println!("Dividindo a expressão em tokens e inserindo em uma lista.");

	let tokens_list: Vec<&str> = lexer(&expr);	// LISTA DE TOKENS

	println!("Imprimindo tokens da lista...");
	println!("{:?}", tokens_list);
	println!(" ");

	println!(" ");
	println!("Criando a pilha de operadores e a fila de saída dos tokens...");
	println!(" ");
	
	let fila: Vec<&str> = parser(tokens_list);

	println!("Imprimindo a notação reversa");
	println!("{:?}", fila);
	println!(" ");

	println!("Criando pilha de execução dos operandos para o cálculo do resultado final");
	
	let resposta = calcular(fila);
	println!("Resposta final = {}", resposta);

}
