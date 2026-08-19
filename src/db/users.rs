use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub password_hash: String,
    pub nome: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub telefone: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Barbeiro {
    pub id: i64,
    pub especialidade: String,
    pub bio_apresentacao: String,
    pub usuario_id: i64,
    pub nome_profissional: String,
    pub ativo: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Barbearia {
    pub id: i64,
    pub nome: String,
    pub endereco: String,
    pub tipo_barbearia: String,
    pub publico: String,
    pub slug: Option<String>,
    pub numero: i64,
    pub complemento: Option<String>,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub cep: String,
    pub telefone: String,
    pub logo_url: Option<String>,
    pub ativo: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BarbeariaMembro {
    pub id: i64,
    pub id_barbearia: i64,
    pub id_barbeiro: i64,
    pub porcentagem_corte: i64,
    pub funcao: String,
    pub ativo: bool,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Produto {
    pub id: i64,
    pub nome: String,
    pub estilo_cobranca: String,
    pub preco_centavos: i64,
    pub id_barbearia: i64,
    pub taxa_de_agendamento_em_centavos: i64,
    pub quantidade_estoque: i64,
    pub descricao: String,
    pub ativado: bool,
    pub imagem_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Servico {
    pub id: i64,
    pub nome: String,
    pub descricao: String,
    pub preco_centavos: i64,
    pub duracao_minutos: i64,
    pub taxa_agendamento_centavos: i64,
    pub ativo: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub barbearia_id: i64,
    pub imagem_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HorarioFuncionamento {
    pub id: i64,
    pub barbearia_id: i64,
    pub dia_da_semana: i64,
    pub hora_inicio: String,
    pub hora_fim: String,
    pub ativo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DisponibilidadeBarbeiro {
    pub id: i64,
    pub barbearia_membro_id: i64,
    pub dia_da_semana: i64,
    pub hora_inicio: String,
    pub hora_fim: String,
    pub ativo: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Agenda {
    pub id: i64,
    pub barbearia_id: i64,
    pub barbeiro_id: i64,
    pub usuario_id: i64,
    pub data_hora_inicio: i64,
    pub data_hora_fim: i64,
    pub status: String,
    pub taxa_agendamento_paga: bool,
    pub servicos_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BarbeariaCliente {
    pub id: i64,
    pub barbearia_id: i64,
    pub usuario_id: i64,
    pub preferencia: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VendaProduto {
    pub id: i64,
    pub barbearia_id: i64,
    pub barbeiro_id: i64,
    pub usuario_id: i64,
    pub agendamento_id: Option<i64>,
    pub quantidade: i64,
    pub preco_unitario_na_venda_em_centavos: i64,
    pub data_venda: i64,
    pub produtos_id: i64,
}
