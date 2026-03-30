fn exibir_nome(nome: &str) {
    println!("Nome: {}", nome);
}

fn adicionar_sufixo(nome: &mut String) {
    nome.push_str(" - atualizado");
}

fn marcar_backup(nome: &mut String) {
    nome.push_str(".bak");
}

fn main() {
    let titulo = String::from("Projeto de Exemplo");

    // Empréstimo imutável: só leitura
    exibir_nome(&titulo);
    println!("Ainda posso usar aqui: {}", titulo);

    // Empréstimo mutável: permite modificar
    let mut descricao = String::from("Conteudo Inicial");

    adicionar_sufixo(&mut descricao);
    println!("Descrição atualizada: {}", descricao);

    // Move
    let categoria = String::from("Documentação");
    let nova_categoria = categoria; // categoria é movida para nova_categoria

    // println!("Categoria original: {}", categoria); // Isso causaria um erro de compilação
    println!("Nova categoria: {}", nova_categoria);

    // Clone
    let etiqueta = String::from("Importante");
    let copia_etiqueta = etiqueta.clone(); // Cria uma cópia de etiqueta

    println!("Etiqueta original: {}", etiqueta);
    println!("Cópia da etiqueta: {}", copia_etiqueta);

    // Empréstimo mutável para marcar backup
    let mut arquivo = String::from("dados.txt");
    marcar_backup(&mut arquivo);
    println!("Arquivo marcado para backup: {}", arquivo);
}
