
fn exibir_sistema(name: &str, versao: i32, ativo: bool) {
    println!("Sistema: {}, Versão: {}, Ativo: {}", name, versao, ativo);
}

fn calcular_registros(base: i32, novos: i32) -> i32 {
    base + novos
}

fn criar_descricao() -> String {
    String::from("Este é um sistema de exemplo.")
}

fn main() {
    // varaiáveis básicas
    let name = "SystemX";
    let versao = 1;

    // Mutável
    let mut registros = 1000;

    registros = calcular_registros(registros, 500);

    let ativo = true;

    let descricao = criar_descricao();

    exibir_sistema(name, versao, ativo);
    println!("Descrição: {}", descricao);
    println!("Total de registros: {}", registros);
}
