use std::io::{self, Write};
use std::thread;
use std::time::Duration;
#[derive(Debug)]
struct Processo{
	pid: i32,
	menSize: i32,
	timeExecution: i32,
}

impl Processo{
	fn new(pid: i32, menSize: i32, timeExecution: i32)-> Processo{
			Processo{
				pid,
				menSize,
				timeExecution,
			}
	}
	
	fn resumo(&self){
		println!("PID {}", self.pid)
	}
	fn executar(&self) {
		println!("Começando a execução do processo PID: {}.\n", self.pid);
		for i in 0..self.timeExecution {
			println!(
				"Progresso: [{}] {}/{} segundos.\r",
				"=".repeat((i + 1) as usize),
				i + 1,
				self.timeExecution
			);
			io::stdout().flush().unwrap();
			thread::sleep(Duration::from_secs(1));
		}
		println!("\nProcesso PID: {} concluído.\n", self.pid);
	}
}

struct Pilha{
	numProcessos:i32,
	vecProcesso: Vec<Processo>,
	proxProcesso:i32,
	totalMen:i32,
	totalTime:i32,
}

impl Pilha{
	fn new() -> Pilha{
		Pilha{
			vecProcesso: Vec:: new(),
			proxProcesso: 0,
			numProcessos: 0,
			totalMen: 0,
			totalTime: 0,
		}
	}

	fn push(&mut self, menSize: i32, timeExecution: i32) {
		let processo = Processo {
			pid: self.proxProcesso,
			menSize,
			timeExecution,
		};
		self.vecProcesso.push(processo);
		self.proxProcesso += 1;
		self.totalMen += menSize;
		self.totalTime += timeExecution;
	}

	fn print_pids(&self) {
		for processo in &self.vecProcesso {
			println!("PID: {}", processo.pid);
		}
	}

	fn executar_processos(&mut self){
		let iterations = self.vecProcesso.len();
		for i in 0..iterations{
			let current_process = &self.vecProcesso[self.vecProcesso.len() - 1];
			current_process.executar();
			self.vecProcesso.pop();
		}
		println!("Total de memória utilizada: {} MB.\nTempo total de execução: {} segundos.", self.totalMen, self.totalTime);
	}
}


fn main(){
	let mut pilha = Pilha::new();

	loop{
		let mut memo_entry = String::new();
		let mut time_entry = String::new();
		println!("Digite o tamanho de memória e o tempo de execução do processo.\n Para finalizar insira 99 99.");
		println!("Informe o valor de memória a ser alocado: ");
		io::stdin().read_line(&mut memo_entry).expect("Por favor informe um valor válido");
		println!("Informe o tempo de execução do programa (entre 30 e 90): ");
		io::stdin().read_line(&mut time_entry).expect("Por favor informe um valor válido");

		if memo_entry.trim() == "99" && time_entry.trim() == "99" {
			break;
		}
	
		let memoria: i32 = memo_entry.trim().parse().expect("Falha ao parsear o tamanho de memória");
		let tempo: i32 = time_entry.trim().parse().expect("Falha ao parsear o tempo de execução");
	
		if tempo < 30 || tempo > 90 {
			println!("O tempo de execução deve estar entre 30 e 90 segundos.");
			continue;
		}
		pilha.push(memoria, tempo);
	}
	pilha.print_pids();
	pilha.executar_processos();
}

