use std::thread::sleep;
use std::time::Duration;
use crate::pagamento::pagamento::Pagamento;

pub struct PagamentoCartao {
    valor_bruto: f64,
    valor_adicionado: Option<f64>,
    valor_liquido: Option<f64>
}
impl PagamentoCartao {
    pub fn new(valor: f64) -> Self {
        PagamentoCartao {
            valor_bruto: valor,
            valor_adicionado: None,
            valor_liquido: None,
        }
    }
}

impl Pagamento for PagamentoCartao {
    fn calcular_pagamento(&mut self) {
        let valor_adicionado = self.valor_bruto * 0.05f64;
        self.valor_adicionado = Some(valor_adicionado);
        self.valor_liquido = Some(
            self.valor_bruto + valor_adicionado
        );
    }

    fn calcular_recibo(&mut self) -> String {
        println!("Pagamento ainda não calculado \n calculando...");
        sleep(Duration::new(1, 500));
        self.calcular_pagamento();
        self.emitir_recibo()
    }

    fn formatar_recibo(&mut self) -> String {
        format!(
            "Pagamento em Cartão\n___________________\n|valor bruto: {} \n|valor adicionado: {} \n|valor líquido: {}",
            self.valor_bruto, self.valor_adicionado.unwrap(), self.valor_liquido.unwrap()
        )
    }

    fn emitir_recibo(&mut self) -> String
    {
        if self.valor_liquido.is_none() || self.valor_adicionado.is_none()
            { self.calcular_recibo() }
        else
            { self.formatar_recibo() }
    }
}