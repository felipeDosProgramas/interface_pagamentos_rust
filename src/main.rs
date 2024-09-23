use crate::pagamento::{
    pagamento::Pagamento,
    pagamento_cartao::PagamentoCartao,
    pagamento_dinheiro::PagamentoDinheiro
};

mod pagamento;

fn instanciar_pagamentos() -> (impl Pagamento, impl Pagamento) { (
    PagamentoCartao::new(100f64),
    PagamentoDinheiro::new(100f64)
) }

fn main() {
    let (mut pc, mut pd) = instanciar_pagamentos();
    println!(
        "\n\n{}\n\n{}",
        pc.emitir_recibo(),
        pd.emitir_recibo()
    );
}
