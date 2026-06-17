#[cfg(test)]
mod lexer_tests {
	use engine::frontend::lexer::Lexer;
	use engine::shared::TokenKind;
	use memory_stats::memory_stats;
	use std::fs;
	use std::time::Instant;

	fn format_int(num: usize) -> String {
		let mut s = num.to_string();
		if s.len() <= 3 {
			return s;
		}
		let mut i = s.len() - 3;
		loop {
			s.insert(i, ' ');
			if i < 3 { break; }
			i -= 3;
		}
		s
	}

	fn format_float(value: f64, frac_digits: usize) -> String {
		let sign = if value.is_sign_negative() { "-" } else { "" };
		let abs  = value.abs();
		let int_part = abs.trunc() as u64;
		let frac_part = abs.fract();
		let int_str = format_int(int_part as usize);
		if frac_digits == 0 {
			return format!("{sign}{int_str}");
		}
		let frac_full = format!("{:.*}", frac_digits, frac_part);
		let frac_str  = frac_full.trim_start_matches('0');
		format!("{sign}{int_str}{frac_str}")
	}

	#[test]
	fn lexer_speed_test_full() {
		// ─────────────────────────────────────────────
		// Source loading
		// ─────────────────────────────────────────────
		let path = "../../files/big.lacon";
		let source_bytes: Vec<u8> = fs::read(path)
			.unwrap_or_else(|_| panic!("Cannot read test file: {path}"));
		let source_size = source_bytes.len();

		let iterations        = 1000;
		let warmup_iterations = 10;
		let multiple_files    = true;
		let iters_to_run      = if multiple_files { iterations } else { 1 };

		// ─────────────────────────────────────────────
		// Warmup
		// ─────────────────────────────────────────────
		let mut lexer = Lexer::new(&source_bytes);
		for _ in 0..warmup_iterations {
				lexer.reset(&source_bytes);
				let tokens = lexer.tokenize();
				std::hint::black_box(tokens);
		}

		// ─────────────────────────────────────────────
		// Benchmark
		// ─────────────────────────────────────────────
		let mem_before   = memory_stats().map_or(0, |m| m.physical_mem);
		let start_time   = Instant::now();
		let mut total_tokens = 0usize;
		let mut total_lines  = 0usize;

		let mut lexer = Lexer::new(&source_bytes);
		for _ in 0..iters_to_run {
				lexer.reset(&source_bytes);
				let tokens = lexer.tokenize();
				total_tokens += tokens.len();
				total_lines  += tokens.iter().filter(|t| t.kind == TokenKind::Newline).count();
				std::hint::black_box(tokens);
		}

		let duration     = start_time.elapsed();
		let mem_after    = memory_stats().map_or(0, |m| m.physical_mem);

		// ─────────────────────────────────────────────
		// Metrics
		// ─────────────────────────────────────────────
		let total_bytes      = source_size * iters_to_run;
		let actual_avg_tokens = total_tokens as f64 / iters_to_run as f64;
		let duration_secs    = duration.as_secs_f64();

		let byte_speed  = if duration_secs > 0.0 { (total_bytes  as f64 / 1_048_576.0) / duration_secs } else { 0.0 };
		let token_speed = if duration_secs > 0.0 { (total_tokens as f64 / 1_000_000.0) / duration_secs } else { 0.0 };
		let lines_speed = if duration_secs > 0.0 { (total_lines  as f64 / 1_000_000.0) / duration_secs } else { 0.0 };

		// ─────────────────────────────────────────────
		// Output
		// ─────────────────────────────────────────────
		println!("========================================");
		println!("ТЕСТИРОВАНИЕ БАЙТ-ЛЕКСЕРА (u8):");
		println!("Режим:             {}", if multiple_files { "Множество файлов" } else { "Один гигантский файл" });
		println!("Длина исходника:   {} байт", format_int(source_size));
		println!("Итераций:          {}", format_int(iters_to_run));
		println!("----------------------------------------");
		println!("РЕЗУЛЬТАТЫ:");
		println!("Всего токенов (Σ):  {}", format_int(total_tokens));
		println!("Всего байт (Σ):    {}", format_int(total_bytes));
		println!("Всего строк (Σ):   {}", format_int(total_lines));
		println!("Среднее токенов/итерацию: {}", format_float(actual_avg_tokens, 2));
		println!("----------------------------------------");
		println!("ПРОИЗВОДИТЕЛЬНОСТЬ:");
		println!("Общее время:        {:.3}ms", duration_secs * 1000.0);
		println!("Среднее на прогон:  {:.3}µs", duration.as_nanos() as f64 / iters_to_run as f64 / 1000.0);
		println!("Память (diff):      {:.2}MB", (mem_after as f64 - mem_before as f64) / 1024.0 / 1024.0);
		println!("СКОРОСТЬ (MiB/s):   {:.2} млн байт/сек", byte_speed);
		println!("СКОРОСТЬ (токены):  {:.2} млн/сек", token_speed);
		println!("СКОРОСТЬ (строки):  {:.2} млн/сек", lines_speed);
		println!("========================================\n");
	}
}
