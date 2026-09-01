#[derive(Clone, PartialEq)]
pub struct Service {
    pub slug: &'static str,
    pub icon: &'static str,
    pub tag: &'static str,
    pub name: &'static str,
    pub time_estimate: &'static str,
    pub description: &'static str,
    /// Palavras/frases que o assistente usa para reconhecer este serviço em texto livre.
    pub keywords: &'static [&'static str],
    /// Itens do checklist "Antes de começar" (diagnóstico prévio).
    pub requirements: &'static [&'static str],
    /// Se o fluxo passa por verificação facial (habilita o simulador de fallback).
    pub needs_biometrics: bool,
    /// Explicação alternativa em linguagem simples, sem jargão burocrático.
    pub simple_explanation: &'static str,
    /// Nota média de satisfação (0.0 a 5.0), para a seção de transparência de desempenho.
    pub rating: f32,
    /// Tempo médio de conclusão, em dias.
    pub avg_days: u32,
    /// Total de reclamações registradas.
    pub complaints: u32,
    /// Percentual de solicitações resolvidas dentro do prazo previsto.
    pub resolved_pct: u32,
    /// Principal problema relatado pelos usuários.
    pub main_issue: &'static str,
}

#[derive(Clone, PartialEq)]
pub struct Category {
    pub icon: &'static str,
    pub name: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AlertLevel {
    Urgent,
    Warning,
    Info,
}

#[derive(Clone, PartialEq)]
pub struct Alert {
    pub icon: &'static str,
    pub title: &'static str,
    pub message: &'static str,
    pub level: AlertLevel,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LifeState {
    Ok,
    Attention,
    Pending,
}

/// Um "cartão" da vida do cidadão para o painel "Minha vida no governo".
#[derive(Clone, PartialEq)]
pub struct LifeArea {
    pub icon: &'static str,
    pub name: &'static str,
    pub status: &'static str,
    pub state: LifeState,
    pub related_slug: Option<&'static str>,
}

#[derive(Clone, PartialEq)]
pub struct AttendancePoint {
    pub name: &'static str,
    pub address: &'static str,
    pub hours: &'static str,
    pub distance_km: f32,
    pub needs_appointment: bool,
    pub services: &'static [&'static str],
}

/// Um motivo de bloqueio para o botão "Não consigo resolver", com a solução sugerida.
#[derive(Clone, PartialEq)]
pub struct HelpReason {
    pub icon: &'static str,
    pub label: &'static str,
    pub solution: &'static str,
}

/// Faixa de ação de um sistema antifraude, conforme a pontuação de risco combinada.
#[derive(Clone, Copy, PartialEq)]
pub enum FraudRiskLevel {
    Normal,
    Monitoring,
    Verification,
    Blocked,
}

/// Um fator individual avaliado de forma independente pelo sistema de análise de risco
/// (defesa em profundidade: nenhum sinal isolado decide sozinho).
#[derive(Clone, PartialEq)]
pub struct FraudSignal {
    pub label: &'static str,
    pub detail: &'static str,
    pub ok: bool,
}

pub fn services() -> Vec<Service> {
    vec![
        Service {
            slug: "assinatura-eletronica",
            icon: "✍️",
            tag: "Identidade Digital",
            name: "Assinatura Eletrônica",
            time_estimate: "~5 min",
            description: "Assine documentos oficiais com validade jurídica usando sua conta gov.br, sem precisar imprimir ou reconhecer firma.",
            keywords: &["assinar documento", "assinatura digital", "assinar contrato", "assinar pdf"],
            requirements: &["Conta gov.br ativa", "Documento em PDF para assinar"],
            needs_biometrics: false,
            simple_explanation: "É como assinar um papel, mas no celular ou computador. Vale legalmente, sem precisar imprimir nada.",
            rating: 4.6,
            avg_days: 1,
            complaints: 320,
            resolved_pct: 96,
            main_issue: "Arquivo PDF inválido ou corrompido",
        },
        Service {
            slug: "consultar-imposto-renda",
            icon: "💰",
            tag: "Finanças e Impostos",
            name: "Consultar Imposto de Renda",
            time_estimate: "~3 min",
            description: "Veja o andamento da sua declaração, mensagens da Receita Federal e eventuais pendências.",
            keywords: &["imposto de renda", "declaração", "receita federal", "situação da declaração"],
            requirements: &["Conta gov.br ativa", "CPF regularizado"],
            needs_biometrics: false,
            simple_explanation: "Veja se está tudo certo com o Imposto de Renda que você já entregou.",
            rating: 4.3,
            avg_days: 1,
            complaints: 890,
            resolved_pct: 94,
            main_issue: "Lentidão no sistema em época de pico",
        },
        Service {
            slug: "entregar-imposto-renda",
            icon: "📄",
            tag: "Finanças e Impostos",
            name: "Entregar Imposto de Renda",
            time_estimate: "~20 min",
            description: "Preencha e envie sua declaração anual do Imposto de Renda de Pessoa Física diretamente pela plataforma.",
            keywords: &["declarar imposto de renda", "entregar declaração", "declaração anual", "ir 2026"],
            requirements: &["Conta gov.br nível Prata ou Ouro", "Informe de rendimentos", "CPF regularizado"],
            needs_biometrics: false,
            simple_explanation: "Envie ao governo quanto você ganhou e gastou no ano, para saber se recebe dinheiro de volta ou se deve pagar mais.",
            rating: 3.9,
            avg_days: 3,
            complaints: 2100,
            resolved_pct: 88,
            main_issue: "Dificuldade para preencher os rendimentos",
        },
        Service {
            slug: "consultar-restituicao",
            icon: "🧾",
            tag: "Finanças e Impostos",
            name: "Consultar Restituição",
            time_estimate: "~2 min",
            description: "Consulte o valor e a data de pagamento da sua restituição do Imposto de Renda.",
            keywords: &["restituição", "restituição de imposto", "quando vou receber", "extrato do ir"],
            requirements: &["Conta gov.br ativa", "CPF regularizado"],
            needs_biometrics: false,
            simple_explanation: "Veja se o governo vai te devolver dinheiro do Imposto de Renda, e quando.",
            rating: 4.5,
            avg_days: 1,
            complaints: 410,
            resolved_pct: 97,
            main_issue: "Valor divergente do esperado",
        },
        Service {
            slug: "carteira-trabalho",
            icon: "🪪",
            tag: "Trabalho e Previdência",
            name: "Carteira de Trabalho Digital",
            time_estimate: "~10 min",
            description: "Acesse seu histórico profissional, contratos de trabalho e solicite a emissão da carteira digital.",
            keywords: &["carteira de trabalho", "ctps", "carteira digital de trabalho", "vínculos empregatícios"],
            requirements: &["Conta gov.br nível Prata ou Ouro", "Verificação facial (selfie)"],
            needs_biometrics: true,
            simple_explanation: "Veja seus empregos registrados e tire uma carteira de trabalho digital, sem precisar ir a um posto.",
            rating: 3.6,
            avg_days: 5,
            complaints: 3400,
            resolved_pct: 79,
            main_issue: "Verificação facial não reconhece o rosto",
        },
        Service {
            slug: "passaporte",
            icon: "🛂",
            tag: "Viagens e Turismo",
            name: "Solicitar Passaporte",
            time_estimate: "~15 min",
            description: "Agende o atendimento, pague a taxa e acompanhe o andamento da emissão do seu passaporte.",
            keywords: &["passaporte", "tirar passaporte", "viajar para fora", "viagem internacional"],
            requirements: &["Conta gov.br ativa", "CPF regularizado", "Pagamento da taxa (GRU)"],
            needs_biometrics: false,
            simple_explanation: "Documento para viajar para fora do Brasil. Aqui você paga a taxa e marca o dia para tirar a foto e as digitais.",
            rating: 3.8,
            avg_days: 12,
            complaints: 5200,
            resolved_pct: 74,
            main_issue: "Demora para conseguir horário de atendimento",
        },
        Service {
            slug: "cnh-digital",
            icon: "🚗",
            tag: "Trânsito",
            name: "CNH Digital e Segunda Via",
            time_estimate: "~8 min",
            description: "Consulte sua CNH digital, solicite a segunda via em caso de perda ou roubo, ou verifique a situação da sua habilitação.",
            keywords: &[
                "carteira de motorista",
                "cnh",
                "carteira de habilitação",
                "segunda via",
                "perdi minha carteira",
                "perdi minha cnh",
                "habilitação",
            ],
            requirements: &["Conta gov.br nível Prata ou Ouro", "CPF regularizado", "Verificação facial (selfie)"],
            needs_biometrics: true,
            simple_explanation: "Sua carteira de motorista no celular. Se perdeu a de papel ou plástico, peça uma segunda via aqui.",
            rating: 3.7,
            avg_days: 8,
            complaints: 4100,
            resolved_pct: 81,
            main_issue: "Verificação facial não reconhece o rosto",
        },
    ]
}

/// Reconhecimento simples por palavras-chave (sem backend/LLM) usado pelo assistente:
/// compara o texto livre digitado com as keywords cadastradas em cada serviço.
///
/// Keywords de uma palavra só (ex.: "ctps") exigem correspondência EXATA de palavra —
/// isso evita falso positivo do tipo uma keyword curta como "ir" batendo dentro de uma
/// palavra não relacionada como "carteira" (que contém as letras "ir"). Keywords com
/// espaço (frases, ex.: "carteira de trabalho") usam correspondência por substring no
/// texto completo, o que é seguro porque frases longas raramente aparecem por acaso
/// dentro de outro texto.
pub fn match_services(query: &str, services: &[Service]) -> Vec<Service> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    let words: Vec<&str> = q.split_whitespace().collect();
    services
        .iter()
        .filter(|s| {
            s.keywords.iter().any(|k| {
                if k.contains(' ') {
                    q.contains(k)
                } else {
                    words.contains(k)
                }
            })
        })
        .take(3)
        .cloned()
        .collect()
}

pub fn categories() -> Vec<Category> {
    vec![
        Category {
            icon: "🌾",
            name: "Agricultura e Pecuária",
        },
        Category {
            icon: "🤝",
            name: "Assistência Social",
        },
        Category {
            icon: "🔬",
            name: "Ciência e Tecnologia",
        },
        Category {
            icon: "🎓",
            name: "Educação e Pesquisa",
        },
        Category {
            icon: "🏥",
            name: "Saúde e Vigilância Sanitária",
        },
        Category {
            icon: "⚖️",
            name: "Justiça e Segurança",
        },
        Category {
            icon: "🏗️",
            name: "Infraestrutura e Trânsito",
        },
        Category {
            icon: "🌎",
            name: "Meio Ambiente e Clima",
        },
        Category {
            icon: "💼",
            name: "Trabalho e Previdência",
        },
        Category {
            icon: "🏢",
            name: "Empresa, Indústria e Comércio",
        },
        Category {
            icon: "✈️",
            name: "Viagens e Turismo",
        },
        Category {
            icon: "🎭",
            name: "Cultura, Artes e Esportes",
        },
    ]
}

/// Alertas simulados do cidadão (prazos, pendências, valores disponíveis).
pub fn alerts() -> Vec<Alert> {
    vec![
        Alert {
            icon: "⚠️",
            title: "Sua CNH vence em 30 dias",
            message: "Renove agora para evitar multa e pontos na carteira.",
            level: AlertLevel::Urgent,
        },
        Alert {
            icon: "📋",
            title: "Existe uma pendência no seu CPF",
            message: "Regularize para não perder acesso a outros serviços.",
            level: AlertLevel::Warning,
        },
        Alert {
            icon: "💰",
            title: "Você pode ter direito a uma restituição",
            message: "Consulte o valor disponível do seu Imposto de Renda.",
            level: AlertLevel::Info,
        },
        Alert {
            icon: "📅",
            title: "O prazo da declaração termina em 5 dias",
            message: "Entregue o Imposto de Renda até 31/05 para evitar multa.",
            level: AlertLevel::Warning,
        },
    ]
}

/// Estado simulado do cidadão em diferentes áreas da vida, para o painel "Minha vida no governo".
pub fn life_areas() -> Vec<LifeArea> {
    vec![
        LifeArea {
            icon: "🪪",
            name: "Documentos",
            status: "CPF e identidade regulares",
            state: LifeState::Ok,
            related_slug: None,
        },
        LifeArea {
            icon: "🚗",
            name: "CNH e Veículos",
            status: "CNH vence em 30 dias",
            state: LifeState::Attention,
            related_slug: Some("cnh-digital"),
        },
        LifeArea {
            icon: "💰",
            name: "Impostos",
            status: "Declaração de 2026 ainda pendente",
            state: LifeState::Pending,
            related_slug: Some("entregar-imposto-renda"),
        },
        LifeArea {
            icon: "🧾",
            name: "Restituição",
            status: "R$ 1.240,00 disponível para saque",
            state: LifeState::Attention,
            related_slug: Some("consultar-restituicao"),
        },
        LifeArea {
            icon: "💼",
            name: "Trabalho",
            status: "3 vínculos empregatícios registrados",
            state: LifeState::Ok,
            related_slug: Some("carteira-trabalho"),
        },
        LifeArea {
            icon: "🏛️",
            name: "Benefícios",
            status: "Nenhum benefício ativo no momento",
            state: LifeState::Ok,
            related_slug: None,
        },
        LifeArea {
            icon: "🗳️",
            name: "Situação Eleitoral",
            status: "Título regular e em dia",
            state: LifeState::Ok,
            related_slug: None,
        },
        LifeArea {
            icon: "🏥",
            name: "Saúde",
            status: "Cartão do SUS ativo",
            state: LifeState::Ok,
            related_slug: None,
        },
        LifeArea {
            icon: "🎓",
            name: "Educação",
            status: "Nenhum registro acadêmico vinculado",
            state: LifeState::Ok,
            related_slug: None,
        },
        LifeArea {
            icon: "🛂",
            name: "Passaporte",
            status: "Nenhum passaporte emitido",
            state: LifeState::Ok,
            related_slug: Some("passaporte"),
        },
        LifeArea {
            icon: "✍️",
            name: "Assinaturas",
            status: "2 documentos assinados este ano",
            state: LifeState::Ok,
            related_slug: Some("assinatura-eletronica"),
        },
        LifeArea {
            icon: "👴",
            name: "Aposentadoria",
            status: "Simulação de tempo de contribuição disponível",
            state: LifeState::Ok,
            related_slug: None,
        },
    ]
}

/// Postos de atendimento presencial simulados, para quem não consegue resolver online.
pub fn attendance_points() -> Vec<AttendancePoint> {
    vec![
        AttendancePoint {
            name: "Unidade de Atendimento gov.br — Centro",
            address: "Praça da Sé, 100 — Centro",
            hours: "Seg. a sex., 8h às 17h",
            distance_km: 1.2,
            needs_appointment: true,
            services: &["CNH", "Passaporte", "Carteira de Trabalho"],
        },
        AttendancePoint {
            name: "Poupatempo — Zona Norte",
            address: "Av. Norte, 850 — Zona Norte",
            hours: "Seg. a sáb., 7h às 19h",
            distance_km: 4.7,
            needs_appointment: true,
            services: &["CNH", "Documentos", "Assinatura de documentos"],
        },
        AttendancePoint {
            name: "Correios — Agência Jardim das Flores",
            address: "Rua das Flores, 45 — Jardim das Flores",
            hours: "Seg. a sex., 9h às 18h",
            distance_km: 2.5,
            needs_appointment: false,
            services: &["Passaporte", "Reconhecimento de firma"],
        },
    ]
}

/// Motivos de bloqueio do botão "Não consigo resolver", com a solução sugerida para cada um.
pub fn help_reasons() -> Vec<HelpReason> {
    vec![
        HelpReason {
            icon: "🔑",
            label: "Não consigo entrar",
            solution: "Confira se está digitando o CPF correto. Se o problema continuar, use \"Esqueci minha senha\" na tela de login.",
        },
        HelpReason {
            icon: "🔒",
            label: "Esqueci minha senha",
            solution: "Clique em \"Esqueci minha senha\" na tela de login e siga a verificação por e-mail, SMS ou banco credenciado.",
        },
        HelpReason {
            icon: "🤳",
            label: "Reconhecimento facial falhou",
            solution: "Você pode confirmar sua identidade por banco credenciado, e-mail, telefone ou atendimento presencial — sem precisar da selfie.",
        },
        HelpReason {
            icon: "📄",
            label: "Não tenho o documento pedido",
            solution: "Veja abaixo o posto de atendimento mais próximo para emitir o documento, ou continue depois: seus dados ficam salvos.",
        },
        HelpReason {
            icon: "❓",
            label: "Não entendi a etapa",
            solution: "Ative o botão \"Linguagem simples\" na página do serviço para uma explicação mais direta, sem termos técnicos.",
        },
        HelpReason {
            icon: "🐛",
            label: "O sistema apresentou erro",
            solution: "Tente novamente em alguns minutos. Se o erro continuar, abra um chamado na Central de Ajuda com o horário em que ocorreu.",
        },
        HelpReason {
            icon: "⏳",
            label: "Meu pedido está parado",
            solution: "Consulte o tempo médio de conclusão na página do serviço. Se já passou do prazo, você pode registrar uma reclamação formal.",
        },
        HelpReason {
            icon: "🗣️",
            label: "Preciso falar com uma pessoa",
            solution: "Ligue para a Central 0800 000 0000 ou procure um dos postos de atendimento presencial listados abaixo.",
        },
    ]
}

/// Pontuação de risco simulada (0-100), combinando vários sinais independentes —
/// nunca uma regra única decide sozinha se a conta é suspeita.
pub fn fraud_risk_score() -> i32 {
    34
}

/// Classifica a pontuação em uma faixa de ação, como em um sistema antifraude real.
pub fn fraud_level(score: i32) -> FraudRiskLevel {
    match score {
        0..=30 => FraudRiskLevel::Normal,
        31..=60 => FraudRiskLevel::Monitoring,
        61..=80 => FraudRiskLevel::Verification,
        _ => FraudRiskLevel::Blocked,
    }
}

/// Sinais independentes que compõem a pontuação de risco (comportamento, tentativas de
/// burlar regras, velocidade, múltiplas contas, inconsistências, abuso e automação).
pub fn fraud_signals() -> Vec<FraudSignal> {
    vec![
        FraudSignal {
            label: "Comportamento de navegação",
            detail: "Padrão de uso compatível com seu histórico nos últimos 90 dias.",
            ok: true,
        },
        FraudSignal {
            label: "Tentativas de burlar regras",
            detail: "Nenhuma tentativa de contornar verificação de identidade detectada.",
            ok: true,
        },
        FraudSignal {
            label: "Velocidade das ações",
            detail: "Ritmo de preenchimento mais rápido que o normal nesta sessão.",
            ok: false,
        },
        FraudSignal {
            label: "Múltiplas contas",
            detail: "Apenas 1 conta gov.br associada a este CPF.",
            ok: true,
        },
        FraudSignal {
            label: "Consistência dos dados",
            detail: "Nenhuma divergência entre dados cadastrais e uso da conta.",
            ok: true,
        },
        FraudSignal {
            label: "Histórico de abuso",
            detail: "Nenhum registro de bloqueio ou penalidade anterior.",
            ok: true,
        },
        FraudSignal {
            label: "Sinais de automação",
            detail: "Nenhum padrão de acesso automatizado (bot) identificado.",
            ok: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Invariantes básicas dos dados mock ---------------------------------------

    #[test]
    fn services_is_not_empty_and_has_unique_slugs() {
        let list = services();
        assert!(!list.is_empty());

        let mut slugs: Vec<&str> = list.iter().map(|s| s.slug).collect();
        slugs.sort_unstable();
        slugs.dedup();
        assert_eq!(
            slugs.len(),
            list.len(),
            "cada serviço deve ter um slug único"
        );
    }

    #[test]
    fn services_needing_biometrics_mention_facial_verification() {
        for s in services().iter().filter(|s| s.needs_biometrics) {
            assert!(
                s.requirements.iter().any(|r| r.contains("facial")),
                "serviço {} marca needs_biometrics mas não lista verificação facial nos requirements",
                s.slug
            );
        }
    }

    #[test]
    fn categories_alerts_and_life_areas_are_not_empty() {
        assert!(!categories().is_empty());
        assert!(!alerts().is_empty());
        assert!(!life_areas().is_empty());
        assert!(!attendance_points().is_empty());
        assert!(!help_reasons().is_empty());
        assert!(!fraud_signals().is_empty());
    }

    // --- fraud_level: faixas de decisão do "antifraude" ----------------------------

    #[test]
    fn fraud_level_boundaries() {
        assert!(matches!(fraud_level(0), FraudRiskLevel::Normal));
        assert!(matches!(fraud_level(30), FraudRiskLevel::Normal));
        assert!(matches!(fraud_level(31), FraudRiskLevel::Monitoring));
        assert!(matches!(fraud_level(60), FraudRiskLevel::Monitoring));
        assert!(matches!(fraud_level(61), FraudRiskLevel::Verification));
        assert!(matches!(fraud_level(80), FraudRiskLevel::Verification));
        assert!(matches!(fraud_level(81), FraudRiskLevel::Blocked));
        assert!(matches!(fraud_level(100), FraudRiskLevel::Blocked));
    }

    #[test]
    fn fraud_risk_score_is_classified_as_monitoring() {
        // fraud_risk_score() hoje retorna 34, que deve cair na faixa "Monitoring".
        assert!(matches!(
            fraud_level(fraud_risk_score()),
            FraudRiskLevel::Monitoring
        ));
    }

    // --- match_services: reconhecimento por palavras-chave do assistente -----------

    #[test]
    fn match_services_empty_or_blank_query_returns_nothing() {
        let list = services();
        assert!(match_services("", &list).is_empty());
        assert!(match_services("   ", &list).is_empty());
    }

    #[test]
    fn match_services_matches_single_word_keyword_by_whole_word() {
        let list = services();
        let result = match_services("ctps", &list);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].slug, "carteira-trabalho");
    }

    #[test]
    fn match_services_matches_multi_word_phrase_keyword() {
        let list = services();
        let result = match_services("perdi minha carteira de motorista", &list);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].slug, "cnh-digital");
    }

    /// Regressão: uma keyword curta como "ir" não pode "vazar" e casar dentro de uma
    /// palavra não relacionada que só contém essas letras por acaso (ex.: "carteIRa").
    /// A keyword real associada a "ir" é a frase "ir 2026", então uma consulta que não
    /// contém essa frase exata não deve casar com `entregar-imposto-renda`.
    #[test]
    fn match_services_does_not_false_positive_on_substring_of_short_keyword() {
        let list = services();
        let result = match_services("perdi minha carteira de trabalho", &list);
        assert!(
            result.iter().all(|s| s.slug != "entregar-imposto-renda"),
            "consulta sobre carteira de trabalho não deveria casar com entregar-imposto-renda"
        );
        assert!(result.iter().any(|s| s.slug == "carteira-trabalho"));
    }

    #[test]
    fn match_services_respects_take_3_limit() {
        // "conta gov.br ativa" aparece nos requirements (não keywords) de vários
        // serviços; aqui garantimos apenas que o resultado nunca ultrapassa 3 itens,
        // não importa quantos serviços a keyword combine.
        let list = services();
        let result = match_services("passaporte cnh carteira de trabalho restituição", &list);
        assert!(result.len() <= 3);
    }
}
