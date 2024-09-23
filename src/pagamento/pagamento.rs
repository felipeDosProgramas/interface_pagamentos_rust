pub trait Pagamento {
    fn calcular_pagamento(&mut self);
    fn calcular_recibo(&mut self) -> String;
    fn formatar_recibo(&mut self) -> String;
    fn emitir_recibo(&mut self) -> String;
}