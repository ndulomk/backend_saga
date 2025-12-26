use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use rayon::prelude::*;



fn main() {
    println!("Calculadora Paralela");
    
    let total_numbers = 100_000_000u64;
    let start_time = Instant::now();
    
    let ((squares, roots), logs) = rayon::join(
        || {
            rayon::join(
                || {
                    println!("Calculando quadrados...");
                    (1u64..=total_numbers)
                        .into_par_iter()
                        .take_any(5)  
                        .map(|n| {
                            let n_f64 = n as f64;
                            (n_f64, n_f64 * n_f64)
                        })
                        .collect::<Vec<(f64, f64)>>()
                },
                || {
                    println!("Calculando raízes quadradas...");
                    (1u64..=total_numbers)
                        .into_par_iter()
                        .take_any(5)
                        .map(|n| {
                            let n_f64 = n as f64;
                            (n_f64, n_f64.sqrt())
                        })
                        .collect::<Vec<(f64, f64)>>()
                },
            )
        },
        || {
            println!("Calculando logaritmos...");
            (1u64..=total_numbers)
                .into_par_iter()
                .take_any(5)
                .map(|n| {
                    let n_f64 = n as f64;
                    (n_f64, n_f64.ln())
                })
                .collect::<Vec<(f64, f64)>>()
        },
    );
    
    let elapsed_time = start_time.elapsed();
    
    println!("\n=== RESULTADOS ===");
    println!("Tempo total (primeiros 5 de cada): {:.2?}", elapsed_time);
    
    println!("\nQuadrados (primeiros 5):");
    for (input, output) in squares {
        println!("  {} -> {}", input, output);
    }
    
    println!("\nRaízes Quadradas (primeiros 5):");
    for (input, output) in roots {
        println!("  {} -> {:.6}", input, output);
    }
    
    println!("\nLogaritmos (primeiros 5):");
    for (input, output) in logs {
        println!("  {} -> {:.6}", input, output);
    }
    
    calcular_estatisticas_otimizado(total_numbers);
    benchmark_performance_real(total_numbers);
}

fn calcular_estatisticas_otimizado(total_numbers: u64) {
    println!("\n=== ESTATÍSTICAS OTIMIZADAS ===");
    
    let start = Instant::now();
    let n = total_numbers as f64;
    
    let soma = n * (n + 1.0) / 2.0;
    println!("Soma total: {}", soma);
    
    let media = (n + 1.0) / 2.0;
    println!("Média: {:.2}", media);
    
    println!("Máximo: {}", n);
    println!("Mínimo: 1");
    
    let variance = (n * n - 1.0) / 12.0;
    println!("Variância: {:.2}", variance);
    println!("Desvio Padrão: {:.2}", variance.sqrt());
    
    println!("Tempo cálculo estatísticas: {:?}", start.elapsed());
}

fn benchmark_performance_real(total_numbers: u64) {
    println!("\n=== BENCHMARK REAL DE PERFORMANCE ===");
    
    let start = Instant::now();
    let counter = AtomicUsize::new(0);
    let limit = 1_000_000;
    
    let results: Vec<(f64, f64, f64)> = (1u64..=total_numbers)
        .into_par_iter()
        .filter_map(|n| {
            let idx = counter.fetch_add(1, Ordering::Relaxed);
            if idx < limit {
                let n_f64 = n as f64;
                Some((n_f64 * n_f64, n_f64.sqrt(), n_f64.ln()))
            } else {
                None
            }
        })
        .collect();
    
    println!("Processados {} números em {:?}", results.len(), start.elapsed());
    
    if let Some(last) = results.last() {
        println!("Último resultado - Quadrado: {:.2}, Raiz: {:.6}, Log: {:.6}", 
                last.0, last.1, last.2);
    }
    
    let start2 = Instant::now();
    let sum_squares: f64 = (1u64..=total_numbers)
        .into_par_iter()
        .map(|n| {
            let n_f64 = n as f64;
            n_f64 * n_f64
        })
        .sum();
    
    println!("\nSoma dos quadrados de 1 a {}: {}", total_numbers, sum_squares);
    println!("Tempo para soma dos quadrados: {:?}", start2.elapsed());
    
    let start3 = Instant::now();
    let num_threads = rayon::current_num_threads();
    println!("\nNúmero de threads disponíveis: {}", num_threads);
    
    let chunk_size = (total_numbers / num_threads as u64).max(1);
    
    let max_values: Vec<f64> = (0..num_threads)
        .into_par_iter()
        .map(|i| {
            let start = i as u64 * chunk_size + 1;
            let end = if i == num_threads - 1 {
                total_numbers
            } else {
                (i as u64 + 1) * chunk_size
            };
            
            let mut max_val = f64::NEG_INFINITY;
            for n in start..=end {
                let val = (n as f64).ln();
                if val > max_val {
                    max_val = val;
                }
            }
            max_val
        })
        .collect();
    
    let overall_max = max_values.into_iter().fold(f64::NEG_INFINITY, f64::max);
    println!("Máximo logaritmo encontrado: {:.6}", overall_max);
    println!("Tempo para encontrar máximo: {:?}", start3.elapsed());
}