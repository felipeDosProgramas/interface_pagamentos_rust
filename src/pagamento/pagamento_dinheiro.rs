use std::thread::sleep;
use std::time::Duration;
use crate::pagamento::pagamento::Pagamento;

pub struct PagamentoDinheiro {
    valor_bruto: f64,
    valor_descontado: Option<f64>,
    valor_liquido: Option<f64>
}
impl PagamentoDinheiro {
    pub fn new(valor: f64) -> PagamentoDinheiro {
        PagamentoDinheiro{
            valor_bruto: valor,
            valor_descontado: None,
            valor_liquido: None
        }
    }
}
impl Pagamento for PagamentoDinheiro {
    fn calcular_pagamento(&mut self) {
        let valor_adicionado = self.valor_bruto * 0.1f64;
        self.valor_descontado = Some(valor_adicionado);
        self.valor_liquido = Some(
            self.valor_bruto - valor_adicionado
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
            "Pagamento em dinheiro\n_____________________\n|valor bruto: {} \n|valor descontado: {} \n|valor líquido: {}",
            self.valor_bruto, self.valor_descontado.unwrap(), self.valor_liquido.unwrap()
        )
    }

    fn emitir_recibo(&mut self) -> String
    {
        if self.valor_liquido.is_none() || self.valor_descontado.is_none()
            { self.calcular_recibo() }
        else
            { self.formatar_recibo() }
    }
}
