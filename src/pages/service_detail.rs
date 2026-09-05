use dioxus::prelude::*;

use crate::accessibility::speak;
use crate::components::{FeedbackForm, Icon, IconKind, PerformanceCard};
use crate::data::services;
use crate::routes::Route;

#[derive(Clone, Copy, PartialEq)]
enum FaceCheck {
    Idle,
    /// Câmera ativa, rodando a detecção facial real no navegador (Shape Detection API).
    Scanning,
    Failed,
    Resolved,
}

/// Roda a verificação facial real inteiramente no navegador via a Shape
/// Detection API (`FaceDetector`). Nenhum frame de vídeo, imagem ou dado
/// biométrico é enviado ao Rust ou ao servidor — só o resultado final
/// (`success`/`no_face`/`denied`/`unsupported`) atravessa a fronteira JS/Rust.
async fn run_face_detection() -> String {
    let mut eval = document::eval(
        r#"
        (async () => {
            if (!('FaceDetector' in window)) {
                dioxus.send('unsupported');
                return;
            }
            let stream;
            try {
                stream = await navigator.mediaDevices.getUserMedia({ video: { facingMode: 'user' } });
            } catch (e) {
                dioxus.send('denied');
                return;
            }
            const video = document.getElementById('face-video');
            video.srcObject = stream;
            try {
                await video.play();
            } catch (e) {
                // alguns navegadores exigem interação do usuário; seguimos mesmo assim
            }

            const detector = new FaceDetector({ fastMode: true, maxDetectedFaces: 1 });
            const canvas = document.createElement('canvas');
            canvas.width = video.videoWidth || 320;
            canvas.height = video.videoHeight || 240;
            const ctx = canvas.getContext('2d');

            let found = false;
            const deadline = Date.now() + 8000;
            while (Date.now() < deadline) {
                try {
                    ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
                    const faces = await detector.detect(canvas);
                    if (faces && faces.length > 0) {
                        found = true;
                        break;
                    }
                } catch (e) {
                    // erro transitório de detecção: continua tentando até o prazo
                }
                await new Promise((r) => setTimeout(r, 300));
            }

            stream.getTracks().forEach((t) => t.stop());
            video.srcObject = null;

            dioxus.send(found ? 'success' : 'no_face');
        })();
        "#,
    );
    let result: Result<String, _> = eval.recv().await;
    result.unwrap_or_else(|_| "error".to_string())
}

const PROCESS_STEPS: [&str; 6] = [
    "Solicitação enviada",
    "Documentos analisados",
    "Em processamento",
    "Aguardando órgão",
    "Aprovado",
    "Concluído",
];

const ALT_VERIFICATIONS: [(IconKind, &str); 4] = [
    (IconKind::Landmark, "Banco credenciado"),
    (IconKind::Mail, "E-mail"),
    (IconKind::Phone, "Telefone"),
    (IconKind::Building, "Atendimento presencial"),
];

/// Página de detalhe de um serviço: explicação, checklist de pré-requisitos,
/// verificação de identidade simulada (com fallback quando o reconhecimento
/// facial falha) e acompanhamento passo a passo da solicitação.
#[component]
pub fn ServiceDetail(slug: String) -> Element {
    let service = services().into_iter().find(|s| s.slug == slug);
    let mut confirmed = use_signal(|| false);
    let mut started = use_signal(|| false);
    let mut face_check = use_signal(|| FaceCheck::Idle);
    let mut face_message = use_signal(String::new);
    let mut step = use_signal(|| 0usize);
    let mut simple_lang = use_signal(|| false);

    // Dispara a detecção real sempre que entramos em estado "Scanning".
    use_effect(move || {
        if face_check() == FaceCheck::Scanning {
            spawn(async move {
                let result = run_face_detection().await;
                match result.as_str() {
                    "success" => {
                        face_check.set(FaceCheck::Resolved);
                        step.set(0);
                    }
                    "unsupported" => {
                        face_message.set(
                            "Seu navegador não tem suporte à verificação facial real. Use um método alternativo abaixo."
                                .to_string(),
                        );
                        face_check.set(FaceCheck::Failed);
                    }
                    "denied" => {
                        face_message.set(
                            "Não conseguimos acessar sua câmera (permissão negada). Tente novamente ou use um método alternativo."
                                .to_string(),
                        );
                        face_check.set(FaceCheck::Failed);
                    }
                    "no_face" => {
                        face_message.set(
                            "Não conseguimos identificar seu rosto a tempo. Tente novamente com boa iluminação."
                                .to_string(),
                        );
                        face_check.set(FaceCheck::Failed);
                    }
                    _ => {
                        face_message
                            .set("Ocorreu um erro inesperado na verificação facial.".to_string());
                        face_check.set(FaceCheck::Failed);
                    }
                }
            });
        }
    });

    rsx! {
        div { class: "max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-16",
            if let Some(s) = service {
                Link {
                    to: Route::Home {},
                    class: "flex items-center gap-1.5 text-sm font-medium text-govbr-blue hover:underline w-fit",
                    Icon { kind: IconKind::ArrowLeft, class: "w-4 h-4" }
                    "Voltar para a página inicial"
                }
                div { class: "mt-6 flex items-start gap-4",
                    Icon { kind: s.icon, class: "w-10 h-10 text-govbr-blue shrink-0" }
                    div {
                        span { class: "text-xs font-semibold text-govbr-blue uppercase tracking-wide", "{s.tag}" }
                        h1 { class: "text-3xl font-bold text-govbr-blue-dark", "{s.name}" }
                    }
                }
                p { class: "mt-4 text-govbr-gray-text leading-relaxed",
                    if simple_lang() { "{s.simple_explanation}" } else { "{s.description}" }
                }
                div { class: "mt-3 flex flex-wrap items-center gap-3",
                    button {
                        class: "flex items-center gap-1.5 text-xs font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-3 py-1.5",
                        onclick: move |_| simple_lang.toggle(),
                        if simple_lang() {
                            "Ver texto oficial"
                        } else {
                            Icon { kind: IconKind::BookOpen, class: "w-3.5 h-3.5" }
                            "Linguagem simples"
                        }
                    }
                    button {
                        class: "flex items-center gap-1.5 text-xs font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-3 py-1.5",
                        onclick: move |_| {
                            let text = if simple_lang() { s.simple_explanation } else { s.description };
                            speak(text);
                        },
                        Icon { kind: IconKind::Speaker, class: "w-3.5 h-3.5" }
                        "Ouvir"
                    }
                }
                div { class: "mt-4 inline-flex items-center gap-2 text-sm font-medium text-govbr-green bg-govbr-green/10 rounded px-4 py-2",
                    Icon { kind: IconKind::Clock, class: "w-4 h-4" }
                    "Tempo estimado: {s.time_estimate}"
                }
                PerformanceCard { service: s.clone() }

                if !started() {
                    // Diagnóstico prévio: evita que o cidadão descubra no meio do processo que falta algo.
                    div { class: "mt-8 border border-govbr-gray-border rounded-lg p-6",
                        h2 { class: "text-base font-semibold text-govbr-blue-dark mb-1", "Antes de começar" }
                        p { class: "text-sm text-govbr-gray-text mb-4",
                            "Confira se você tem tudo isso pronto, para não ter surpresas no meio do caminho:"
                        }
                        ul { class: "flex flex-col gap-2 mb-4",
                            for req in s.requirements.iter().copied() {
                                li { class: "flex items-center gap-2 text-sm text-govbr-gray-text",
                                    Icon { kind: IconKind::Check, class: "w-4 h-4 text-govbr-green shrink-0" }
                                    "{req}"
                                }
                            }
                        }
                        label { class: "flex items-center gap-2 text-sm font-medium text-govbr-blue-dark cursor-pointer",
                            input {
                                r#type: "checkbox",
                                checked: confirmed(),
                                onchange: move |e| confirmed.set(e.checked()),
                            }
                            "Confirmo que tenho os itens acima"
                        }
                    }

                    button {
                        class: if confirmed() { "mt-6 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-6 py-3" } else { "mt-6 text-sm font-semibold text-white bg-govbr-gray-border cursor-not-allowed rounded-full px-6 py-3" },
                        disabled: !confirmed(),
                        onclick: move |_| {
                            started.set(true);
                            if !s.needs_biometrics {
                                step.set(0);
                            }
                        },
                        "Iniciar: {s.name}"
                    }
                } else if s.needs_biometrics && face_check() != FaceCheck::Resolved {
                    // Verificação facial real (Shape Detection API) com fallback (dor real relatada por usuários do gov.br)
                    div { class: "mt-8 border border-govbr-gray-border rounded-lg p-6 text-center",
                        if face_check() == FaceCheck::Idle {
                            Icon { kind: IconKind::Camera, class: "w-10 h-10 text-govbr-blue mx-auto" }
                            h2 { class: "mt-3 text-base font-semibold text-govbr-blue-dark", "Verificação facial necessária" }
                            p { class: "mt-1 text-sm text-govbr-gray-text",
                                "Vamos usar a câmera do seu dispositivo para confirmar sua identidade. Nenhuma imagem ou dado biométrico sai do seu navegador."
                            }
                            button {
                                class: "mt-4 text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-6 py-2.5",
                                onclick: move |_| face_check.set(FaceCheck::Scanning),
                                "📷 Verificar com a câmera"
                            }
                        } else if face_check() == FaceCheck::Scanning {
                            span { class: "text-4xl", "📷" }
                            h2 { class: "mt-3 text-base font-semibold text-govbr-blue-dark", "Analisando..." }
                            p { class: "mt-1 text-sm text-govbr-gray-text", "Posicione seu rosto dentro da câmera. Isso pode levar alguns segundos." }
                            video {
                                id: "face-video",
                                class: "mt-4 mx-auto rounded-lg border border-govbr-gray-border w-64 h-48 object-cover bg-black",
                                autoplay: true,
                                "muted": "true",
                                "playsinline": "true",
                            }
                            p { class: "mt-2 text-xs text-govbr-gray-text",
                                "🔒 A verificação acontece só no seu navegador — nenhuma imagem é enviada ou armazenada."
                            }
                        } else {
<<<<<<< HEAD
                            Icon { kind: IconKind::Warning, class: "w-10 h-10 text-amber-600 mx-auto" }
                            h2 { class: "mt-3 text-base font-semibold text-govbr-blue-dark", "O reconhecimento facial não funcionou" }
                            p { class: "mt-1 text-sm text-govbr-gray-text", "Sem problemas. Vamos tentar outra forma de confirmar sua identidade:" }
                            div { class: "flex flex-wrap justify-center gap-2 mt-4",
                                for (icon , label) in ALT_VERIFICATIONS {
=======
                            span { class: "text-4xl", "⚠️" }
                            h2 { class: "mt-3 text-base font-semibold text-govbr-blue-dark", "Não foi possível confirmar pela câmera" }
                            p { class: "mt-1 text-sm text-govbr-gray-text", "{face_message}" }
                            div { class: "flex flex-wrap justify-center gap-2 mt-4",
                                button {
                                    class: "text-sm font-semibold text-white bg-govbr-blue hover:bg-govbr-blue-light transition-colors rounded-full px-4 py-2",
                                    onclick: move |_| face_check.set(FaceCheck::Scanning),
                                    "🔄 Tentar novamente com a câmera"
                                }
                                for alt in ["🏦 Banco credenciado", "📧 E-mail", "📞 Telefone", "🏢 Atendimento presencial"] {
>>>>>>> c09a621e3ecc8da34ca00dd2db84b30738ee7099
                                    button {
                                        class: "flex items-center gap-2 text-sm font-medium text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-4 py-2",
                                        onclick: move |_| {
                                            face_check.set(FaceCheck::Resolved);
                                            step.set(0);
                                        },
                                        Icon { kind: icon, class: "w-4 h-4" }
                                        "{label}"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Acompanhamento visual do processo (reduz a ansiedade do "será que deu certo?")
                    div { class: "mt-8 border border-govbr-gray-border rounded-lg p-6",
                        h2 { class: "text-base font-semibold text-govbr-blue-dark mb-5", "Acompanhamento da solicitação" }
                        div { class: "flex flex-col",
                            for (i , label) in PROCESS_STEPS.iter().enumerate() {
                                div { class: "flex items-start gap-3",
                                    div { class: "flex flex-col items-center",
                                        span {
                                            class: if i <= step() { "flex items-center justify-center w-7 h-7 rounded-full bg-govbr-green text-white text-xs font-bold shrink-0" } else { "flex items-center justify-center w-7 h-7 rounded-full bg-govbr-gray-border text-govbr-gray-text text-xs font-bold shrink-0" },
                                            if i <= step() {
                                                Icon { kind: IconKind::Check, class: "w-3.5 h-3.5" }
                                            } else {
                                                "{i + 1}"
                                            }
                                        }
                                        if i < PROCESS_STEPS.len() - 1 {
                                            div { class: if i < step() { "w-px flex-1 min-h-6 bg-govbr-green" } else { "w-px flex-1 min-h-6 bg-govbr-gray-border" } }
                                        }
                                    }
                                    p {
                                        class: if i <= step() { "text-sm font-medium text-govbr-blue-dark pb-6" } else { "text-sm text-govbr-gray-text pb-6" },
                                        "{label}"
                                    }
                                }
                            }
                        }
                        if step() < PROCESS_STEPS.len() - 1 {
                            button {
                                class: "text-sm font-semibold text-govbr-blue border border-govbr-blue hover:bg-govbr-blue/5 transition-colors rounded-full px-5 py-2",
                                onclick: move |_| step.set((step() + 1).min(PROCESS_STEPS.len() - 1)),
                                "Simular avanço da etapa"
                            }
                        } else {
                            p { class: "flex items-center gap-1.5 text-sm font-semibold text-govbr-green",
                                Icon { kind: IconKind::CheckCircle, class: "w-4 h-4" }
                                "Processo concluído com sucesso!"
                            }
                            FeedbackForm {}
                        }
                    }
                }
            } else {
                div { class: "text-center",
                    h1 { class: "text-2xl font-bold text-govbr-blue-dark", "Serviço não encontrado" }
                    p { class: "mt-2 text-govbr-gray-text", "O serviço que você procura não existe ou foi removido." }
                    Link {
                        to: Route::Home {},
                        class: "mt-4 inline-flex items-center gap-1.5 text-govbr-blue hover:underline",
                        Icon { kind: IconKind::ArrowLeft, class: "w-4 h-4" }
                        "Voltar para a página inicial"
                    }
                }
            }
        }
    }
}
